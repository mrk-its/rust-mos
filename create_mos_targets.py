import json
import os
import subprocess
import sys

CFG_FILES = {
    "sim": "sim.cfg",
    "c64": "commodore/64.cfg",
    "a800xl": "atari/800xl.cfg",
}

def create_target(target_spec, sdk_build_dir, platform):
    opts = target_spec.copy()
    opts.pop("is-builtin")    
    # opts["linker"] = f"mos-{platform}-clang"   
    config_path = CFG_FILES[platform]
    opts["pre-link-args"] = {
        "gcc": [
            "--config",
            os.path.join(sdk_build_dir, config_path),
        ]
    }
    opts["vendor"] = platform
    return opts

if __name__ == "__main__":

    target_spec = json.loads(
        subprocess.getoutput('rustup run mos rustc --target mos-unknown-none -Z unstable-options --print target-spec-json')
    )

    sdk_build_dir = sys.argv[1]
    dest_dir = sys.argv[2]

    for arch in ("a800xl", "c64", "sim"):
        target_def = create_target(target_spec, sdk_build_dir, arch)
        with open(os.path.join(dest_dir, f"mos-{arch}-none.json"), "w") as fp:
            json.dump(target_def, fp, indent=2)
