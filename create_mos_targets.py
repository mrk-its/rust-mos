import json
import os
import subprocess
import sys

CFG_FILES = {
    "sim": "sim.cfg",
    "c64": "commodore/64.cfg",
    "a800xl": "atari/800xl.cfg",
}

def create_target(target_spec, platform):
    opts = target_spec.copy()
    opts.pop("is-builtin")    
    opts["linker"] = f"mos-{platform}-clang"
    opts["vendor"] = platform
    return opts

if __name__ == "__main__":

    target_spec = json.loads(
        subprocess.getoutput('rustup run mos rustc --target mos-unknown-none -Z unstable-options --print target-spec-json')
    )

    dest_dir = sys.argv[1]

    for arch in ("a800xl", "c64", "sim"):
        target_def = create_target(target_spec, arch)
        target_file = os.path.join(dest_dir, f"mos-{arch}-none.json")
        with open(target_file, "w") as fp:
            json.dump(target_def, fp, indent=2)
            print("created", target_file)
