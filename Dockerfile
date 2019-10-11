FROM ubuntu:18.04
ARG release=elements-0.18.1.1

RUN apt-get update && apt-get install -y \
    wget && \
    cd /opt && \
    wget https://github.com/ElementsProject/elements/releases/download/$release/$release-x86_64-linux-gnu.tar.gz && \
    tar zxvf $release-x86_64-linux-gnu.tar.gz && \
    cd $release && \
    ln -s /opt/$release/bin/elementsd /usr/bin/elementsd && \
    ln -s /opt/$release/bin/elements-cli /usr/bin/elements-cli && \
    sed -i -e '$a alias ecli="elements-cli -chain=liquidregtest -rpcuser=elements -rpcpassword=elements"' /root/.bashrc
