// use crate::spec::{LinkerFlavor, SanitizerSet, StackProbeType, Target};
use crate::spec::{
    //LinkerFlavor,
    Target, TargetOptions, PanicStrategy
};

pub fn target() -> Target {
    // base.pre_link_args.entry(LinkerFlavor::Gcc).or_default().push("-m64".to_string());
    // don't use probe-stack=inline-asm until rust#83139 and rust#84667 are resolved
    // base.stack_probes = StackProbeType::Call;
    // base.supported_sanitizers =
    //    SanitizerSet::ADDRESS | SanitizerSet::LEAK | SanitizerSet::MEMORY | SanitizerSet::THREAD;

    let options = TargetOptions {
        c_int_width: "16".to_string(),
        cpu: "mos6502".to_string(),
        executables: true,
        singlethread: true,
        atomic_cas: false,
        min_atomic_width: Some(8),
        max_atomic_width: Some(8),
        disable_redzone: true,
        panic_strategy: PanicStrategy::Abort,
        linker: Some("clang".to_string()),
        no_default_libraries: false,
        ..Default::default()
    };

    // options.pre_link_args.entry(LinkerFlavor::Gcc).or_default().push("--config /home/mrk/private/llvm-mos-sdk/build/atari/800xl.cfg".to_string());

    Target {
        llvm_target: "mos-unknown-none".to_string(),
        options,
        pointer_width: 16,
        data_layout: "e-m:e-p:16:8-i16:8-i32:8-i64:8-f32:8-f64:8-a:8-Fi8-n8".to_string(),
        arch: "mos".to_string(),
    }
}
