########################################
#
# BUILD
#
#   docker build -t solsa .
#
# RUN
#
#   docker run -it --rm -v $(pwd):/src:ro solsa example_contract.sol
#
# ALIAS
#
#   function solsa () { docker run -it --rm -v $(pwd):/src:ro solsa $@ }
#
########################################

# TODO restructure and cleanup

FROM ubuntu:xenial

########################################
#
# Download build/install dependencies
#
########################################

RUN apt update

RUN apt install -y \
    software-properties-common \
    python \
    curl \
    wget \
    python-pip \
    unzip

RUN curl -sL https://deb.nodesource.com/setup_8.x | bash -

RUN apt install -y nodejs

########################################
#
# Install solc
#
########################################

RUN add-apt-repository -y ppa:ethereum/ethereum
RUN apt update
RUN apt install -y ethereum solc

########################################
#
# Install oyente
#
########################################

RUN python -m pip install oyente

########################################
#
# Install solium
#
########################################

RUN npm install -g solium

########################################
#
# Install echidna
#
########################################

RUN apt install -y \
        git \
        libgmp-dev \
        libbz2-dev \
        libreadline-dev

WORKDIR /tmp
RUN git clone https://github.com/trailofbits/echidna.git echidna
WORKDIR /tmp/echidna

RUN curl -sSL https://get.haskellstack.org/ | sh

RUN stack upgrade
RUN stack setup
RUN stack install

# echidna fails without proper locales

RUN apt install -y \
        locales

ENV LANG en_US.UTF-8
ENV LANGUAGE en_US.UTF-8
ENV LC_ALL en_US.UTF-8

RUN locale-gen en_US.UTF-8

########################################
#
# Install MAIAN
#
########################################

RUN mkdir /tmp/z3
WORKDIR /tmp/z3

# TODO check if z3-prover installed through pip works as well

ENV Z3_VERSION 4.6.0
ENV UBUNTU_VERSION 16.04

RUN wget https://github.com/Z3Prover/z3/releases/download/z3-${Z3_VERSION}/z3-${Z3_VERSION}-x64-ubuntu-${UBUNTU_VERSION}.zip
RUN unzip z3-${Z3_VERSION}-x64-ubuntu-${UBUNTU_VERSION}.zip -d /tmp/
RUN cp -ar /tmp/z3-${Z3_VERSION}-x64-ubuntu-${UBUNTU_VERSION}/bin /tmp/z3-${Z3_VERSION}-x64-ubuntu-${UBUNTU_VERSION}/include /usr/local/

WORKDIR /opt/

RUN python -m pip install web3

RUN git clone https://github.com/MAIAN-tool/MAIAN.git

RUN apt install -y \
        psmisc \
        lsof

########################################
#
# Install mythril
#
########################################

RUN apt install -y \
        libssl-dev \
        python3-pip

RUN python3 -m pip install mythril

########################################
#
# Prepare running environment
#
########################################

COPY .soliumrc.json /etc/.soliumrc.json

COPY run_analysis.sh /opt/run_analysis.sh

ENTRYPOINT ["/opt/run_analysis.sh"]