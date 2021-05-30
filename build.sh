./x.py build -i --stage 0 src/tools/cargo
./x.py build -i && (
  ln -s ../../stage0-tools-bin/cargo build/x86_64-unknown-linux-gnu/stage1/bin/cargo
  rustup toolchain link mos build/x86_64-unknown-linux-gnu/stage1
  rustup default mos
  mkdir -p $RUST_TARGET_PATH
  python3 create_mos_targets.py $RUST_TARGET_PATH
)
