FROM rust:latest

RUN apt-get update && \
    apt-get install -y git clang cmake && \
    cd /opt && \
    git clone https://github.com/blockstream/electrs && \
    cd electrs && \
    git checkout new-index && \
    cargo build --release --features liquid --bin electrs
