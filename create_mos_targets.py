import glob
import json
import os
import subprocess
import sys
import re

def create_target(target_spec, platform):
    opts = target_spec.copy()
    opts.pop("is-builtin")    
    opts["linker"] = f"mos-{platform}-clang"
    opts["vendor"] = platform
    return opts

def get_mos_platforms():
    return sorted(
        re.search("mos-(.*)-clang", i).group(1)
        for i in glob.glob(os.path.dirname(subprocess.getoutput("which mos-atari8-clang")) + '/mos-*-clang')
        if 'common' not in i
    )

if __name__ == "__main__":

    target_spec = json.loads(
        subprocess.getoutput('rustup run mos rustc --target mos-unknown-none -Z unstable-options --print target-spec-json')
    )

    dest_dir = sys.argv[1]

    for arch in get_mos_platforms():
        target_def = create_target(target_spec, arch)
        target_file = os.path.join(dest_dir, f"mos-{arch}-none.json")
        with open(target_file, "w") as fp:
            json.dump(target_def, fp, indent=2)
            print("created", target_file)
