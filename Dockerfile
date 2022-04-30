FROM ubuntu:20.04 as builder

## Install build dependencies.
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y cmake clang curl git make
RUN curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN ${HOME}/.cargo/bin/rustup default nightly
RUN ${HOME}/.cargo/bin/cargo install -f cargo-fuzz

## Add source code to the build stage.
ADD . /stronghold.rs
WORKDIR /stronghold.rs

#RUN ${HOME}/.cargo/bin/cargo build
#RUN cd engine && ${HOME}/.cargo/bin/cargo build

RUN cd engine/fuzz && ${HOME}/.cargo/bin/cargo fuzz build snapshot

# Package Stage
FROM ubuntu:20.04

COPY --from=builder /stronghold.rs/engine/fuzz/target/x86_64-unknown-linux-gnu/release/snapshot /
