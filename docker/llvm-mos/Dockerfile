FROM ubuntu:22.04 as llvm_mos_base
RUN apt-get update
RUN DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends -y \
    git curl ca-certificates ninja-build cmake build-essential  \
    libstdc++-10-dev libxml2-dev libssl-dev pkg-config python3-pip \
    swig python3-dev libedit-dev libncurses5-dev  liblzma-dev

FROM llvm_mos_base as build

WORKDIR /tmp
COPY llvm-mos llvm-mos

WORKDIR llvm-mos

RUN ls -la /usr/lib/
RUN uname -m

RUN cmake -C clang/cmake/caches/MOS.cmake -G "Ninja" -S llvm -B build \
          -DLLVM_INSTALL_TOOLCHAIN_ONLY=OFF \
          -DLLVM_BUILD_LLVM_DYLIB=ON -DLLVM_LINK_LLVM_DYLIB=ON \
          -DLLVM_INSTALL_UTILS=ON -DLLVM_BUILD_UTILS=ON -DLLVM_TOOLCHAIN_UTILITIES=FileCheck \
          -DLLVM_TOOLCHAIN_TOOLS="llvm-addr2line;llvm-ar;llvm-cxxfilt;llvm-dwarfdump;llvm-mc;llvm-nm;llvm-objcopy;llvm-objdump;llvm-ranlib;llvm-readelf;llvm-readobj;llvm-size;llvm-strings;llvm-strip;llvm-symbolizer;llvm-config;llc" \
          -DLIBXML2_LIBRARY=/usr/lib/$(uname -m)-linux-gnu/libxml2.so \
          -DLLVM_TARGETS_TO_BUILD="MOS;X86;AArch64" \
          -DLLVM_ENABLE_PROJECTS="clang;lld;lldb"

RUN cmake --build build -t install

WORKDIR /tmp
COPY llvm-mos-sdk llvm-mos-sdk

WORKDIR  llvm-mos-sdk
RUN cmake -G "Ninja" -B build
RUN cmake --build build -t install

FROM llvm_mos_base

COPY --from=build /usr/local /usr/local

WORKDIR /

RUN apt install -y vim vim-runtime less gawk

WORKDIR /usr/local/src
RUN git clone https://github.com/jhallen/atari-tools
WORKDIR /usr/local/src/atari-tools
RUN make; cp /usr/local/src/atari-tools/atr /usr/local/bin/atr

ARG USERNAME=mos
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Create the user
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME -s /bin/bash \
    #
    # [Optional] Add sudo support. Omit if you don't need to install software after connecting.
    && apt-get update \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

USER $USERNAME
WORKDIR /home/$USERNAME
