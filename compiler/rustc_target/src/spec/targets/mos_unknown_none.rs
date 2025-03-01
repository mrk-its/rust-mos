// use crate::spec::{LinkerFlavor, SanitizerSet, StackProbeType, Target};
use crate::spec::{
    //LinkerFlavor,
    Target, TargetMetadata, TargetOptions, PanicStrategy
};

pub fn target() -> Target {
    // base.pre_link_args.entry(LinkerFlavor::Gcc).or_default().push("-m64".to_string());
    // don't use probe-stack=inline-asm until rust#83139 and rust#84667 are resolved
    // base.stack_probes = StackProbeType::Call;
    // base.supported_sanitizers =
    //    SanitizerSet::ADDRESS | SanitizerSet::LEAK | SanitizerSet::MEMORY | SanitizerSet::THREAD;

    let llvm_args = &[
        "--force-precise-rotation-cost",
        "--jump-inst-cost=6",
        "--force-loop-cold-block",
        "--phi-node-folding-threshold=0",
        "--two-entry-phi-node-folding-threshold=0",
        "--align-large-globals=false",
        "--disable-spill-hoist",
    ];

    let llvm_args = llvm_args.iter().map(|opt| opt.to_string().into()).collect();

    let options = TargetOptions {
        c_int_width: "16".into(),
        cpu: "mos6502".into(),
        executables: true,
        singlethread: true,
        atomic_cas: false,
        min_atomic_width: Some(8),
        max_atomic_width: Some(8),
        disable_redzone: true,
        panic_strategy: PanicStrategy::Abort,
        linker: Some("mos-clang".into()),
        no_default_libraries: false,
        requires_lto: true,
        supports_stack_protector: false,
        trap_unreachable: false,
        llvm_args,
        ..Default::default()
    };

    // options.pre_link_args.entry(LinkerFlavor::Gcc).or_default().push("--config /home/mrk/private/llvm-mos-sdk/build/atari/800xl.cfg".to_string());

    Target {
        llvm_target: "mos-unknown-none".into(),
        options,
        pointer_width: 16,
        data_layout: "e-m:e-p:16:8-p1:8:8-i16:8-i32:8-i64:8-f32:8-f64:8-a:8-Fi8-n8".into(),
        arch: "mos".into(),
        metadata: TargetMetadata {
            description: Some("8-bit 6502".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
    }
}
