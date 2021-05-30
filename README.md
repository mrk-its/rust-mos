<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-light.svg">
    <img alt="The Rust Programming Language: A language empowering everyone to build reliable and efficient software"
         src="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-light.svg"
         width="50%">
  </picture>

[Website][Rust] | [Getting started] | [Learn] | [Documentation] | [Contributing]
</div>

This is the main source code repository for [Rust]. It contains the compiler,
standard library, and documentation.

[Rust]: https://www.rust-lang.org/
[Getting Started]: https://www.rust-lang.org/learn/get-started
[Learn]: https://www.rust-lang.org/learn
[Documentation]: https://www.rust-lang.org/learn#learn-use
[Contributing]: CONTRIBUTING.md

## Why Rust?

- **Performance:** Fast and memory-efficient, suitable for critical services, embedded devices, and easily integrated with other languages.

- **Reliability:** Our rich type system and ownership model ensure memory and thread safety, reducing bugs at compile-time.

- **Productivity:** Comprehensive documentation, a compiler committed to providing great diagnostics, and advanced tooling including package manager and build tool ([Cargo]), auto-formatter ([rustfmt]), linter ([Clippy]) and editor support ([rust-analyzer]).

[Cargo]: https://github.com/rust-lang/cargo
[rustfmt]: https://github.com/rust-lang/rustfmt
[Clippy]: https://github.com/rust-lang/rust-clippy
[rust-analyzer]: https://github.com/rust-lang/rust-analyzer

# MOS target notes

MOS target depends on [llvm-mos](https://github.com/llvm-mos/llvm-mos) and [llvm-mos-sdk](https://github.com/llvm-mos/llvm-mos-sdk). Installation of llvm-mos is described in project's [README](https://github.com/llvm-mos/llvm-mos/blob/main/README.md), but it needs to be slightly modified:

```
git clone https://github.com/llvm-mos/llvm-mos
cd llvm-mos
# For MacOS, use `-DLIBXML2_LIBRARY=/usr/local/opt/libxml2/lib/libxml2.dylib` instead of `-DLIBXML2_LIBRARY=/usr/lib/x86_64-linux-gnu/libxml2.so`
cmake -C clang/cmake/caches/MOS.cmake -G "Ninja" -S llvm -B build \
   -DLLVM_INSTALL_TOOLCHAIN_ONLY=OFF \
   -DLLVM_BUILD_LLVM_DYLIB=ON -DLLVM_LINK_LLVM_DYLIB=ON \
   -DLLVM_INSTALL_UTILS=ON -DLLVM_BUILD_UTILS=ON -DLLVM_TOOLCHAIN_UTILITIES=FileCheck \
   -DLLVM_TOOLCHAIN_TOOLS="llvm-addr2line;llvm-ar;llvm-cxxfilt;llvm-dwarfdump;llvm-mc;llvm-nm;llvm-objcopy;llvm-objdump;llvm-ranlib;llvm-readelf;llvm-readobj;llvm-size;llvm-strings;llvm-strip;llvm-symbolizer;llvm-config;llc" \
   -DLIBXML2_LIBRARY=/usr/lib/x86_64-linux-gnu/libxml2.so \
   -DLLVM_TARGETS_TO_BUILD="MOS;X86" \
   -DLLVM_ENABLE_PROJECTS="clang;lld;lldb"
cmake --build build -t install
```

Build & install llvm-mos-sdk:

```
git clone https://github.com/llvm-mos/llvm-mos-sdk
cd llvm-mos-sdk
cmake -G "Ninja" -B build
cmake --build build -t install
```

And finally build rust mos toolchain:
```
   export RUST_TARGET_PATH=/usr/local/rust-mos-targets/
   cp config.toml.example config.toml
   # in config.toml adjust path to llvm-config
   # if llvm-mos is installed to other than /usr/local prefix
   ./x.py build -i --stage 0 src/tools/cargo
   ./x.py build -i && (
   ln -s ../../stage0-tools-bin/cargo build/x86_64-unknown-linux-gnu/stage1/bin/cargo
      rustup toolchain link mos build/x86_64-unknown-linux-gnu/stage1
      rustup default mos
      mkdir -p $RUST_TARGET_PATH
      python3 create_mos_targets.py $RUST_TARGET_PATH
   )
```

## Quick Start

Read ["Installation"] from [The Book].

["Installation"]: https://doc.rust-lang.org/book/ch01-01-installation.html
[The Book]: https://doc.rust-lang.org/book/index.html

## Installing from Source

If you really want to install from source (though this is not recommended), see
[INSTALL.md](INSTALL.md).

## Getting Help

See https://www.rust-lang.org/community for a list of chat platforms and forums.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Rust is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0), with portions covered by various BSD-like
licenses.

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.

## Trademark

[The Rust Foundation][rust-foundation] owns and protects the Rust and Cargo
trademarks and logos (the "Rust Trademarks").

If you want to use these names or brands, please read the
[media guide][media-guide].

Third-party logos may be subject to third-party copyrights and trademarks. See
[Licenses][policies-licenses] for details.

[rust-foundation]: https://foundation.rust-lang.org/
[media-guide]: https://foundation.rust-lang.org/policies/logo-policy-and-media-guide/
[policies-licenses]: https://www.rust-lang.org/policies/licenses
