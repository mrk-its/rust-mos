ARG TARGETARCH
FROM --platform=linux/amd64 ubuntu:22.04 as cross_build_base

ARG APT_MIRROR=http://ftp.icm.edu.pl/pub/Linux/ubuntu/
ARG INSTALL_PREFIX="/usr/local"

#RUN sed -i s%http://archive.ubuntu.com/ubuntu/%${APT_MIRROR}%g /etc/apt/sources.list
RUN apt-get update

RUN DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends -y \
    sudo git curl ca-certificates ninja-build cmake build-essential \
    libstdc++-10-dev libxml2-dev libssl-dev pkg-config python3-pip \
    swig python3-dev libedit-dev libncurses5-dev liblzma-dev libstdc++6 \
    gcc-11-aarch64-linux-gnu g++-11-aarch64-linux-gnu libstdc++6-arm64-cross

FROM cross_build_base as main-amd64
FROM cross_build_base as final-amd64

FROM cross_build_base as main-arm64

ENV CC=aarch64-linux-gnu-gcc-11
ENV CXX=aarch64-linux-gnu-g++-11
ENV CROSS_CMAKE_ARGS="-DCMAKE_SYSTEM_NAME=Linux -DCMAKE_CXX_FLAGS='-march=armv8.5-a'"

FROM --platform=linux/arm64 ubuntu:22.04 as final-arm64
RUN apt-get update

RUN DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends -y \
    sudo git curl ca-certificates build-essential \
    libstdc++-10-dev libxml2-dev libssl-dev pkg-config \
    python3 \
    libedit-dev libncurses5-dev liblzma-dev libstdc++6

FROM main-${TARGETARCH} as build

COPY llvm-mos /tmp/llvm-mos
WORKDIR /tmp/llvm-mos

RUN ln -s /usr/aarch64-linux-gnu/lib/ld-linux-aarch64.so.1 /lib/ld-linux-aarch64.so.1
RUN cd /lib; for lib in ld-linux-aarch64.so.1 libstdc++.so.6 libm.so.6 libgcc_s.so.1 libc.so.6; do ln -sf /usr/aarch64-linux-gnu/lib/$lib . ; done
RUN cmake -C clang/cmake/caches/MOS.cmake -G "Ninja" -S llvm -B build \
    ${CROSS_CMAKE_ARGS} -DLLVM_ENABLE_LIBXML2=OFF -DLLVM_TARGETS_TO_BUILD="X86;AArch64" \
    -DLLVM_INSTALL_TOOLCHAIN_ONLY=OFF -DLLVM_TOOLCHAIN_UTILITIES=FileCheck -DLLVM_INSTALL_UTILS=ON \
    -DLLVM_MOS_TOOLCHAIN_TOOLS="llvm-config;llvm-addr2line;llvm-ar;llvm-cxxfilt;llvm-dwarfdump;llvm-mc;llvm-mlb;llvm-nm;llvm-objcopy;llvm-objdump;llvm-ranlib;llvm-readelf;llvm-readobj;llvm-size;llvm-strings;llvm-strip;llvm-symbolizer" \
    -DCMAKE_INSTALL_PREFIX=${INSTALL_PREFIX}
RUN cmake --build build -t install

COPY llvm-mos-sdk /tmp/llvm-mos-sdk
WORKDIR /tmp/llvm-mos-sdk

RUN cmake -G "Ninja" -B build  -DCMAKE_INSTALL_PREFIX=${INSTALL_PREFIX} -DLLVM_MOS_BUILD_EXAMPLES=Off
RUN cmake --build build -t install

FROM final-${TARGETARCH}
COPY --from=build /usr/local /usr/local

ARG USERNAME=mos
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# # Create the user
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME -s /bin/bash \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

USER $USERNAME
WORKDIR /home/$USERNAME
