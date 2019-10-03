FROM ubuntu:18.04

RUN apt-get update && apt-get install -y \
    wget && \
    cd /opt && \
    wget https://github.com/ElementsProject/elements/releases/download/elements-0.17.0.2/elements-0.17.0.2-x86_64-linux-gnu.tar.gz && \
    tar zxvf elements-0.17.0.2-x86_64-linux-gnu.tar.gz && \
    cd elements-0.17.0 && \
    ln -s /opt/elements-0.17.0/bin/elementsd /usr/bin/elementsd && \
    ln -s /opt/elements-0.17.0/bin/elements-cli /usr/bin/elements-cli && \
    sed -i -e '$a alias ecli="elements-cli -chain=liquidregtest -rpcuser=elements -rpcpassword=elements"' /root/.bashrc

