########################################
#
# BUILD
#
#   docker build -t solsa .
#
# RUN
#
#   docker run -it --rm -v $(pwd):/src:ro solsa -a example_contract.sol
#
# ALIAS
#
#   function solsa () { docker run -it --rm -v $(pwd):/src:ro solsa $@ }
#
# EXAMPLE USAGE
#
#   solsa -a example_contract.sol
#   solsa -t solc -t maian example_contract.sol
#
########################################

FROM ubuntu:xenial

LABEL name solsa
LABEL version 0.0.1

########################################
#
#  Install through distro repos 
#      solc geth nodejs
#
#  Install through pip
#      oyente mythril MAIAN
#
#  Install through npm
#      soilum
#
#  Install through stack
#      echidna
#
########################################

ENV LANG en_US.UTF-8
ENV LANGUAGE en_US.UTF-8
ENV LC_ALL en_US.UTF-8

RUN apt update -qq \
    && apt install -y --no-install-recommends \
        software-properties-common \
        curl \
        locales \
    && locale-gen en_US.UTF-8 \
    && add-apt-repository -y ppa:ethereum/ethereum \
    && curl -sL https://deb.nodesource.com/setup_8.x | bash - \
    && apt install -y --no-install-recommends \
        build-essential \
        ethereum \
        git \
        libbz2-dev \
        libgmp-dev \
        libreadline-dev \
        libssl-dev \
        lsof \
        netbase \
        nodejs \
        psmisc \
        python3 \
        python3-dev \
        python3-pip \
        python3-setuptools \
        python3-wheel \
        solc \
        unzip \
        wget \
   && python3 -m pip install \
        six \
        mythril \
        # pip release is py2 only and no longer reliably installable
        # bug in solc invocation regarding allow-paths keeps us from using upstream
        https://github.com/fmgoncalves/oyente/archive/2080ded0c2b705a67bf50c79c8c561c7e95418fb.zip \
        mythril \
#   && git clone https://github.com/MAIAN-tool/MAIAN.git /opt/MAIAN \
   && npm install -g solium \
   && curl -sSL https://get.haskellstack.org/ | sh \
   && git clone https://github.com/trailofbits/echidna.git /tmp/echidna \
   && cd /tmp/echidna \
   && stack upgrade \
   && stack setup \
   && stack install \
   && stack clean --full \
   && rm -rf /root/.stack \
   && apt remove --purge -y \
        build-essential \
        curl \
        libbz2-dev \
        libgmp-dev \
        libreadline-dev \
        libssl-dev \
        python3-dev \
        python3-pip \
        python3-wheel \
        unzip \
        wget \
   && apt autoremove --purge -y \
   && rm -rf /var/apt/lists/*

########################################
#
# Prepare running environment
#
########################################

COPY .soliumrc.json /etc/.soliumrc.json

RUN touch /etc/.soliumignore

COPY run_analysis.sh /opt/run_analysis.sh

ENTRYPOINT ["/opt/run_analysis.sh"]