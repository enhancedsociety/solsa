# Smart contracts advanced CI pipeline

The goal of this repo is to find a good way of integrating static analisys tools for smart contracts into CI pipelines.
Ideally, this should be be easy to pick up and use by any project using smart contracts (dapps, wallets, etc).

The main challenges to overcome appear to be:
    - seamless installation and configuration of all tools
    - invocation of each tools taking into account its specificities (sometimes on things as simple as where contracts need to be located)
    - resolution of contract dependencies!!! (biggest identified PITA so far)

### Strategy

Create a self contained docker container for which an alias can be easily set so a seamless command line interface is provided.

This way, at little to no effort to the developer a full, detailed analysis of a smart contract can be obtained.

## Key tools for linting and static analisys

    - solc - compile (portable)
    - Solium - lint (depends on npm)
    - Oyente - static analisys (depends on pip)
    - MAIAN - static analisys (depends on pip, must be built from source along with its dependencies)
    - mythril - static analisys (depends on pip)

## Tools in need of experimentation/validation

    - https://github.com/trailofbits/echidna (some trouble getting)

## Install/Run

```
# RUN
    docker run -it --rm -v $(pwd):/src:ro fmgoncalves/solsa:alpha -a example_contract.sol

# ALIAS
    function solsa () { docker run -it --rm -v $(pwd):/src:ro fmgoncalves/solsa:alpha $@ }
```

## Usage

`solsa` is the alias to the full docker command as described at the top of the Dockerfile.

```
filipe@filipe-imp  ~/Development/eth-tooling/solc-travis   master ●  solsa -h                        
Usage:
    /opt/run_analysis.sh -h
                           Display this help message
    /opt/run_analysis.sh -a CONTRACT_PATH
                           Run all tools
    /opt/run_analysis.sh -t TOOL [-t TOOL] CONTRACT_PATH
                           Run selected tools (out of solc,oyente,solium,mythril,echidna,maian)
```

[![asciicast](https://asciinema.org/a/aTU1EpinFsNZsH7yx0SfwLvzu.png)](https://asciinema.org/a/aTU1EpinFsNZsH7yx0SfwLvzu)

[![asciicast](https://asciinema.org/a/mAjh4QSLdr9HsJQF8ftoDjnM0.png)](https://asciinema.org/a/mAjh4QSLdr9HsJQF8ftoDjnM0)

[![asciicast](https://asciinema.org/a/eqxJBhDhZo7TmnkHcnBZRa7sh.png)](https://asciinema.org/a/eqxJBhDhZo7TmnkHcnBZRa7sh)

## TODO

  - [x] Container with chosen static analysis tools
  - [x] Options to selectively enable tools
  - [ ] Setup Docker Hub (or equivalent registry) account for Enhanced Society and push image there
  - [ ] Optimize container size - 4.65GB is **NOT** acceptable (use prebuilt/preinstalled python packages with `alpine` base image)
  - [ ] Automatic contract discovery and analisys in project path (no need to specify which contract to run)
  - [ ] Supress output on success (or add quiet option for it)
  - [ ] Make echidna work consistently
  - [ ] [NEVERENDING] keep finding, evaluating and integrating tools to improve quality of contracts developed