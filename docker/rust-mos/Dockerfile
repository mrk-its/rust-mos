ARG LLVM_MOS_IMAGE
# FROM ${LLVM_MOS_IMAGE}-arm64 as arm64_image
FROM ${LLVM_MOS_IMAGE}-amd64 as builder

# COPY --from=arm64_image /usr /arm64-sysroot/usr
RUN cd /lib; ln -s /usr/aarch64-linux-gnu/lib/ld-linux-aarch64.so.1 .
# RUN cd /usr/lib/aarch64-linux-gnu/; ln -s /arm64-sysroot/usr/lib/aarch64-linux-gnu/libc_nonshared.a .
# RUN cd /lib/aarch64-linux-gnu/; ln -s /arm64-sysroot/usr/lib/aarch64-linux-gnu/libc.so.6
# RUN cd /arm64-sysroot/usr/include/openssl/; ln -s ../aarch64-linux-gnu/openssl/configuration.h; ln -s ../aarch64-linux-gnu/openssl/opensslconf.h .

WORKDIR /tmp
COPY . rust-mos

WORKDIR rust-mos

# hack for x.py dist failure:
RUN mkdir -p /tmp/rust-mos/src/llvm-project/libunwind

RUN QEMU_LD_PREFIX=/usr/aarch64-linux-gnu PKG_CONFIG=$(realpath tools/pkg-config-wrapper.sh) ARM64_SYSROOT=/arm64-sysroot python3 x.py dist

FROM ubuntu:22.04
COPY --from=builder /tmp/rust-mos-build/dist /usr/local/dist

#ENV PATH=/root/.cargo/bin:/usr/local/amd64-unknown-linux-gnu/bin:${PATH}

# ENV RUST_TARGET_PATH=/usr/local/rust-mos-target
# RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain none -y
# RUN rustup toolchain link mos /tmp/rust-mos-build/x86_64-unknown-linux-gnu/stage1
# RUN rustup default mos

# RUN mkdir $RUST_TARGET_PATH
# RUN python3 tools/create_mos_targets.py $RUST_TARGET_PATH

# WORKDIR /

# RUN cp /tmp/rust-mos-build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargo  /tmp/rust-mos-build/x86_64-unknown-linux-gnu/stage1/bin && \
#     mv /tmp/rust-mos-build/x86_64-unknown-linux-gnu/stage1 /usr/local/rust-mos && \
#     rm -fr .git

# RUN find /usr/local/rust-mos -name '*.rlib' -exec chmod a+r {} \;

# FROM --platform=${PLATFORM} ${LLVM_MOS_SDK_IMAGE}

# COPY --from=build /usr/local/src/rust-mos /usr/local/src/rust-mos
# COPY --from=build /usr/local/rust-mos /usr/local/rust-mos

# USER mos
# WORKDIR /home/mos

# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# ENV PATH=/home/mos/.cargo/bin:${PATH}
# RUN rustup toolchain link mos /usr/local/rust-mos
# RUN rustup default mos

# ENV RUST_TARGET_PATH=/home/mos/rust-mos-target
# RUN mkdir $RUST_TARGET_PATH
# RUN python3 /usr/local/src/rust-mos/create_mos_targets.py $RUST_TARGET_PATH

# USER root
# RUN rustup toolchain link mos /usr/local/rust-mos
# RUN rustup default mos

# USER mos
