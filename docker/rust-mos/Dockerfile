ARG LLVM_MOS_SDK_IMAGE=mrkits/llvm-mos-sdk:latest
FROM ${LLVM_MOS_SDK_IMAGE} as build
# RUN apt-get update
# RUN DEBIAN_FRONTEND=noninteractive apt-get install -y python3 libssl-dev libxml2-dev pkg-config 
USER root

WORKDIR /usr/local/src
COPY . rust-mos
WORKDIR rust-mos
COPY docker/rust-mos/config.toml config.toml

RUN python3 x.py build -i --stage 0 src/tools/cargo
RUN python3 x.py build -i

RUN cp /tmp/rust-mos-build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargo  /tmp/rust-mos-build/x86_64-unknown-linux-gnu/stage1/bin && \
    mv /tmp/rust-mos-build/x86_64-unknown-linux-gnu/stage1 /usr/local/rust-mos && \
    rm -fr .git

FROM ${LLVM_MOS_SDK_IMAGE}

COPY --from=build /usr/local/src/rust-mos /usr/local/src/rust-mos
COPY --from=build /usr/local/rust-mos /usr/local/rust-mos

USER mos
WORKDIR /home/mos

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/home/mos/.cargo/bin:${PATH}
RUN rustup toolchain link mos /usr/local/rust-mos
RUN rustup default mos

ENV RUST_TARGET_PATH=/home/mos/rust-mos-target
RUN mkdir $RUST_TARGET_PATH
RUN python3 /usr/local/src/rust-mos/create_mos_targets.py $RUST_TARGET_PATH
