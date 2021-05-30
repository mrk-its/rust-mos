import glob
import json
import os
import subprocess
import sys
import re

def create_target(target_spec, platform, vendor, _os):
    opts = target_spec.copy()
    opts.pop("is-builtin")    
    opts["linker"] = f"mos-{platform}-clang"
    opts["vendor"] = vendor
    opts["os"] = _os
    return opts

def vendor_os(platform):
    platform = platform.split('-', 1)
    assert len(platform) <= 2, f"{platform}"
    if len(platform) == 1:
        platform = platform + ['none']
    return platform

def get_mos_platforms():
    return sorted(
        re.search("mos-(.*)-clang", i).group(1)
        for i in glob.glob(os.path.dirname(subprocess.getoutput("which mos-sim-clang")) + '/mos-*-clang')
        if 'common' not in i
    )

if __name__ == "__main__":

    target_spec = json.loads(
        subprocess.getoutput('rustc --target mos-unknown-none -Z unstable-options --print target-spec-json')
    )

    dest_dir = sys.argv[1]

    for platform in get_mos_platforms():
        vendor, _os = vendor_os(platform)
        target_def = create_target(target_spec, platform, vendor, _os)
        target_file = os.path.join(dest_dir, f"mos-{vendor}-{_os}.json")
        with open(target_file, "w") as fp:
            json.dump(target_def, fp, indent=2)
            print("created", target_file)
