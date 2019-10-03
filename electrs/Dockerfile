FROM rust:latest

RUN apt-get update && \
    apt-get install -y git clang cmake && \
    cd /opt && \
    git clone https://github.com/blockstream/electrs && \
    cd electrs && \
    git checkout new-index && \
    cargo build --release --features liquid --bin electrs

CMD cd /opt/electrs/target/release && ./electrs -vvvv --timestamp --jsonrpc-import --network liquidregtest --cookie elements:elements --daemon-rpc-addr 172.32.0.2:7040
