//! An infrastructure to mechanically analyse proof trees.
//!
//! It is unavoidable that this representation is somewhat
//! lossy as it should hide quite a few semantically relevant things,
//! e.g. canonicalization and the order of nested goals.
//!
//! @lcnr: However, a lot of the weirdness here is not strictly necessary
//! and could be improved in the future. This is mostly good enough for
//! coherence right now and was annoying to implement, so I am leaving it
//! as is until we start using it for something else.

use rustc_ast_ir::try_visit;
use rustc_ast_ir::visit::VisitorResult;
use rustc_infer::infer::resolve::EagerResolver;
use rustc_infer::infer::type_variable::TypeVariableOrigin;
use rustc_infer::infer::{DefineOpaqueTypes, InferCtxt, InferOk};
use rustc_infer::traits::{TraitEngine, TraitEngineExt};
use rustc_macros::extension;
use rustc_middle::infer::unify_key::ConstVariableOrigin;
use rustc_middle::traits::query::NoSolution;
use rustc_middle::traits::solve::{inspect, QueryResult};
use rustc_middle::traits::solve::{Certainty, Goal};
use rustc_middle::traits::ObligationCause;
use rustc_middle::ty;
use rustc_middle::ty::TypeFoldable;
use rustc_span::{Span, DUMMY_SP};

use crate::solve::eval_ctxt::canonical;
use crate::solve::FulfillmentCtxt;
use crate::solve::{EvalCtxt, GoalEvaluationKind, GoalSource};
use crate::solve::{GenerateProofTree, InferCtxtEvalExt};

pub struct InspectConfig {
    pub max_depth: usize,
}

pub struct InspectGoal<'a, 'tcx> {
    infcx: &'a InferCtxt<'tcx>,
    depth: usize,
    orig_values: Vec<ty::GenericArg<'tcx>>,
    goal: Goal<'tcx, ty::Predicate<'tcx>>,
    result: Result<Certainty, NoSolution>,
    evaluation_kind: inspect::CanonicalGoalEvaluationKind<'tcx>,
    /// The expected term of a `NormalizesTo` goal. It gets
    /// replaced with an unconstrained inference variable when
    /// computing `NormalizesTo` goals and we return the nested
    /// goals to the caller, who also equates the actual term
    /// with the expected.
    ///
    /// This is an implementation detail of the trait solver and
    /// not something we want to leak to users. We therefore
    /// treat `NormalizesTo` goals as if they apply the expected
    /// type at the end of each candidate.
    normalizes_to_term_hack: Option<NormalizesToTermHack<'tcx>>,
}

#[derive(Copy, Clone)]
struct NormalizesToTermHack<'tcx> {
    term: ty::Term<'tcx>,
    unconstrained_term: ty::Term<'tcx>,
}

impl<'tcx> NormalizesToTermHack<'tcx> {
    fn relate(
        self,
        infcx: &InferCtxt<'tcx>,
        span: Span,
        param_env: ty::ParamEnv<'tcx>,
    ) -> Result<Certainty, NoSolution> {
        infcx
            .at(&ObligationCause::dummy_with_span(span), param_env)
            .eq(DefineOpaqueTypes::Yes, self.term, self.unconstrained_term)
            .map_err(|_| NoSolution)
            .and_then(|InferOk { value: (), obligations }| {
                let mut fulfill_cx = FulfillmentCtxt::new(infcx);
                fulfill_cx.register_predicate_obligations(infcx, obligations);
                if fulfill_cx.select_where_possible(infcx).is_empty() {
                    if fulfill_cx.pending_obligations().is_empty() {
                        Ok(Certainty::Yes)
                    } else {
                        Ok(Certainty::AMBIGUOUS)
                    }
                } else {
                    Err(NoSolution)
                }
            })
    }
}

pub struct InspectCandidate<'a, 'tcx> {
    goal: &'a InspectGoal<'a, 'tcx>,
    kind: inspect::ProbeKind<'tcx>,
    nested_goals: Vec<inspect::CanonicalState<'tcx, Goal<'tcx, ty::Predicate<'tcx>>>>,
    final_state: inspect::CanonicalState<'tcx, ()>,
    result: QueryResult<'tcx>,
    shallow_certainty: Certainty,
}

impl<'a, 'tcx> InspectCandidate<'a, 'tcx> {
    pub fn kind(&self) -> inspect::ProbeKind<'tcx> {
        self.kind
    }

    pub fn result(&self) -> Result<Certainty, NoSolution> {
        self.result.map(|c| c.value.certainty)
    }

    pub fn goal(&self) -> &'a InspectGoal<'a, 'tcx> {
        self.goal
    }

    /// Certainty passed into `evaluate_added_goals_and_make_canonical_response`.
    ///
    /// If this certainty is `Yes`, then we must be confident that the candidate
    /// must hold iff it's nested goals hold. This is not true if the certainty is
    /// `Maybe(..)`, which suggests we forced ambiguity instead.
    ///
    /// This is *not* the certainty of the candidate's full nested evaluation, which
    /// can be accessed with [`Self::result`] instead.
    pub fn shallow_certainty(&self) -> Certainty {
        self.shallow_certainty
    }

    /// Visit all nested goals of this candidate without rolling
    /// back their inference constraints. This function modifies
    /// the state of the `infcx`.
    pub fn visit_nested_no_probe<V: ProofTreeVisitor<'tcx>>(&self, visitor: &mut V) -> V::Result {
        if self.goal.depth < visitor.config().max_depth {
            for goal in self.instantiate_nested_goals(visitor.span()) {
                try_visit!(visitor.visit_goal(&goal));
            }
        }

        V::Result::output()
    }

    /// Instantiate the nested goals for the candidate without rolling back their
    /// inference constraints. This function modifies the state of the `infcx`.
    pub fn instantiate_nested_goals(&self, span: Span) -> Vec<InspectGoal<'a, 'tcx>> {
        let infcx = self.goal.infcx;
        let param_env = self.goal.goal.param_env;
        let mut orig_values = self.goal.orig_values.to_vec();
        let instantiated_goals: Vec<_> = self
            .nested_goals
            .iter()
            .map(|goal| {
                canonical::instantiate_canonical_state(
                    infcx,
                    span,
                    param_env,
                    &mut orig_values,
                    *goal,
                )
            })
            .collect();

        let () = canonical::instantiate_canonical_state(
            infcx,
            span,
            param_env,
            &mut orig_values,
            self.final_state,
        );

        if let Some(term_hack) = self.goal.normalizes_to_term_hack {
            // FIXME: We ignore the expected term of `NormalizesTo` goals
            // when computing the result of its candidates. This is
            // scuffed.
            let _ = term_hack.relate(infcx, span, param_env);
        }

        instantiated_goals
            .into_iter()
            .map(|goal| match goal.predicate.kind().no_bound_vars() {
                Some(ty::PredicateKind::NormalizesTo(ty::NormalizesTo { alias, term })) => {
                    let unconstrained_term = match term.unpack() {
                        ty::TermKind::Ty(_) => infcx
                            .next_ty_var(TypeVariableOrigin { param_def_id: None, span })
                            .into(),
                        ty::TermKind::Const(ct) => infcx
                            .next_const_var(
                                ct.ty(),
                                ConstVariableOrigin { param_def_id: None, span },
                            )
                            .into(),
                    };
                    let goal =
                        goal.with(infcx.tcx, ty::NormalizesTo { alias, term: unconstrained_term });
                    let proof_tree = EvalCtxt::enter_root(infcx, GenerateProofTree::Yes, |ecx| {
                        ecx.evaluate_goal_raw(GoalEvaluationKind::Root, GoalSource::Misc, goal)
                    })
                    .1;
                    InspectGoal::new(
                        infcx,
                        self.goal.depth + 1,
                        proof_tree.unwrap(),
                        Some(NormalizesToTermHack { term, unconstrained_term }),
                    )
                }
                _ => InspectGoal::new(
                    infcx,
                    self.goal.depth + 1,
                    infcx.evaluate_root_goal(goal, GenerateProofTree::Yes).1.unwrap(),
                    None,
                ),
            })
            .collect()
    }

    /// Visit all nested goals of this candidate, rolling back
    /// all inference constraints.
    pub fn visit_nested_in_probe<V: ProofTreeVisitor<'tcx>>(&self, visitor: &mut V) -> V::Result {
        self.goal.infcx.probe(|_| self.visit_nested_no_probe(visitor))
    }
}

impl<'a, 'tcx> InspectGoal<'a, 'tcx> {
    pub fn infcx(&self) -> &'a InferCtxt<'tcx> {
        self.infcx
    }

    pub fn goal(&self) -> Goal<'tcx, ty::Predicate<'tcx>> {
        self.goal
    }

    pub fn result(&self) -> Result<Certainty, NoSolution> {
        self.result
    }

    fn candidates_recur(
        &'a self,
        candidates: &mut Vec<InspectCandidate<'a, 'tcx>>,
        nested_goals: &mut Vec<inspect::CanonicalState<'tcx, Goal<'tcx, ty::Predicate<'tcx>>>>,
        probe: &inspect::Probe<'tcx>,
    ) {
        let mut shallow_certainty = None;
        for step in &probe.steps {
            match step {
                &inspect::ProbeStep::AddGoal(_source, goal) => nested_goals.push(goal),
                inspect::ProbeStep::NestedProbe(ref probe) => {
                    // Nested probes have to prove goals added in their parent
                    // but do not leak them, so we truncate the added goals
                    // afterwards.
                    let num_goals = nested_goals.len();
                    self.candidates_recur(candidates, nested_goals, probe);
                    nested_goals.truncate(num_goals);
                }
                inspect::ProbeStep::MakeCanonicalResponse { shallow_certainty: c } => {
                    assert_eq!(shallow_certainty.replace(*c), None);
                }
                inspect::ProbeStep::EvaluateGoals(_) => (),
            }
        }

        match probe.kind {
            inspect::ProbeKind::NormalizedSelfTyAssembly
            | inspect::ProbeKind::UnsizeAssembly
            | inspect::ProbeKind::UpcastProjectionCompatibility => (),

            // We add a candidate even for the root evaluation if there
            // is only one way to prove a given goal, e.g. for `WellFormed`.
            inspect::ProbeKind::Root { result }
            | inspect::ProbeKind::TryNormalizeNonRigid { result }
            | inspect::ProbeKind::TraitCandidate { source: _, result }
            | inspect::ProbeKind::OpaqueTypeStorageLookup { result } => {
                // We only add a candidate if `shallow_certainty` was set, which means
                // that we ended up calling `evaluate_added_goals_and_make_canonical_response`.
                if let Some(shallow_certainty) = shallow_certainty {
                    candidates.push(InspectCandidate {
                        goal: self,
                        kind: probe.kind,
                        nested_goals: nested_goals.clone(),
                        final_state: probe.final_state,
                        result,
                        shallow_certainty,
                    });
                }
            }
        }
    }

    pub fn candidates(&'a self) -> Vec<InspectCandidate<'a, 'tcx>> {
        let mut candidates = vec![];
        let last_eval_step = match self.evaluation_kind {
            inspect::CanonicalGoalEvaluationKind::Overflow
            | inspect::CanonicalGoalEvaluationKind::CycleInStack
            | inspect::CanonicalGoalEvaluationKind::ProvisionalCacheHit => {
                warn!("unexpected root evaluation: {:?}", self.evaluation_kind);
                return vec![];
            }
            inspect::CanonicalGoalEvaluationKind::Evaluation { revisions } => {
                if let Some(last) = revisions.last() {
                    last
                } else {
                    return vec![];
                }
            }
        };

        let mut nested_goals = vec![];
        self.candidates_recur(&mut candidates, &mut nested_goals, &last_eval_step.evaluation);

        candidates
    }

    /// Returns the single candidate applicable for the current goal, if it exists.
    ///
    /// Returns `None` if there are either no or multiple applicable candidates.
    pub fn unique_applicable_candidate(&'a self) -> Option<InspectCandidate<'a, 'tcx>> {
        // FIXME(-Znext-solver): This does not handle impl candidates
        // hidden by env candidates.
        let mut candidates = self.candidates();
        candidates.retain(|c| c.result().is_ok());
        candidates.pop().filter(|_| candidates.is_empty())
    }

    fn new(
        infcx: &'a InferCtxt<'tcx>,
        depth: usize,
        root: inspect::GoalEvaluation<'tcx>,
        normalizes_to_term_hack: Option<NormalizesToTermHack<'tcx>>,
    ) -> Self {
        let inspect::GoalEvaluation { uncanonicalized_goal, kind, evaluation } = root;
        let inspect::GoalEvaluationKind::Root { orig_values } = kind else { unreachable!() };

        let result = evaluation.result.and_then(|ok| {
            if let Some(term_hack) = normalizes_to_term_hack {
                infcx
                    .probe(|_| term_hack.relate(infcx, DUMMY_SP, uncanonicalized_goal.param_env))
                    .map(|certainty| ok.value.certainty.unify_with(certainty))
            } else {
                Ok(ok.value.certainty)
            }
        });

        InspectGoal {
            infcx,
            depth,
            orig_values,
            goal: uncanonicalized_goal.fold_with(&mut EagerResolver::new(infcx)),
            result,
            evaluation_kind: evaluation.kind,
            normalizes_to_term_hack,
        }
    }
}

/// The public API to interact with proof trees.
pub trait ProofTreeVisitor<'tcx> {
    type Result: VisitorResult = ();

    fn span(&self) -> Span;

    fn config(&self) -> InspectConfig {
        InspectConfig { max_depth: 10 }
    }

    fn visit_goal(&mut self, goal: &InspectGoal<'_, 'tcx>) -> Self::Result;
}

#[extension(pub trait ProofTreeInferCtxtExt<'tcx>)]
impl<'tcx> InferCtxt<'tcx> {
    fn visit_proof_tree<V: ProofTreeVisitor<'tcx>>(
        &self,
        goal: Goal<'tcx, ty::Predicate<'tcx>>,
        visitor: &mut V,
    ) -> V::Result {
        let (_, proof_tree) = self.evaluate_root_goal(goal, GenerateProofTree::Yes);
        let proof_tree = proof_tree.unwrap();
        visitor.visit_goal(&InspectGoal::new(self, 0, proof_tree, None))
    }
}
