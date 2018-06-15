# Smart contracts tooling aggregator

[![Build Status](https://api.travis-ci.org/enhancedsociety/solsa.svg?branch=master)](https://travis-ci.org/enhancedsociety/solsa)

[![Crate version](https://img.shields.io/crates/v/solsa.svg)](https://crates.io/crates/solsa)
[![Crate version](https://img.shields.io/crates/d/solsa.svg)](https://crates.io/crates/solsa)

[![Solc container Size](https://img.shields.io/microbadger/image-size/enhancedsociety/solc.svg?label=solc+docker+image+size)](https://hub.docker.com/r/enhancedsociety/solc/)
[![Solium container Size](https://img.shields.io/microbadger/image-size/enhancedsociety/solium.svg?label=solium+docker+image+size)](https://hub.docker.com/r/enhancedsociety/solium/)
[![Mythril container Size](https://img.shields.io/microbadger/image-size/enhancedsociety/mythril.svg?label=mythril+docker+image+size)](https://hub.docker.com/r/enhancedsociety/mythril/)
[![Oyente container Size](https://img.shields.io/microbadger/image-size/enhancedsociety/oyente.svg?label=oyente+docker+image+size)](https://hub.docker.com/r/enhancedsociety/oyente/)



The goal of this repo is to find a good way of integrating static analysis tools for smart contracts into CI pipelines.
Ideally, this should be be easy to pick up and use by any project using smart contracts (dapps, wallets, etc).

The main challenges to overcome appear to be:

    - seamless installation and configuration of all tools
    - invocation of each tool taking into account its specifics (sometimes on things as simple as where contracts need to be located)

### Strategy

Create a static binary that invokes minimal self contained docker containers for each tool. Aggregate all invocation results and present them nicely (in a webpage like [solhydra](https://github.com/BlockChainCompany/solhydra) or in a cli as to include in CI pipelines)

This way, at little to no effort to the developer a full, detailed analysis of a smart contract can be obtained.

## Key tools for linting and static analysis

    - solc - compile (portable)
    - Solium - lint (depends on npm)
    - solgraph - function control flow visualiation (depends on npm)
    - Oyente - static analysis (depends on pip)
    - MAIAN - static analysis (python, but no distributable release, must get from source)  (removed due to lack of maintenance)
    - mythril - static analysis (depends on pip)
    - echidna - fuzz tester (haskell compiled binary)

## Install

`solsa` is a rust standalone binary available on [crates.io](https://crates.io/crates/solsa), but it does depend on a few docker images it **expects** to find already installed.


To install the `solsa` command, do
```sh
cargo install solsa
```


To install the docker images it depends on do
```sh
docker pull enhancedsociety/solc
docker pull enhancedsociety/solium
docker pull enhancedsociety/oyente
docker pull enhancedsociety/mythril
```
these images have been optimized for size and ease of use, so they are prepared to be run independently, and are much **much** **MUCH** smaller than their official or naively built counterparts.


## Usage

```
$ solsa -h

solsa 0.1.5
Enhanced Society
Aggregates static analysis tooling for ethereum smart contracts.

USAGE:
    solsa [FLAGS] [OPTIONS] --contract-file <contract-file>

FLAGS:
        --error-exit    Exit with error code if issues are found
    -h, --help          Prints help information
        --html          Output the report as an html file
        --json          Output the report as JSON
    -p, --preload       Preload docker containers necessary for execution
        --silent        Do not output the report, but only basic pass/fail info
    -V, --version       Prints version information

OPTIONS:
    -f, --contract-file <contract-file>    Path to Solidity smart contract
    -d, --depth <depth>                    Depth of analysis, the deeper the more thorough, but also the slower
                                           [default: shallow]  [possible values: shallow, deep]
    -o <output>                            File to write report into
```


Example run

```
$ solsa -f contracts/BurnableCrowdsaleToken.sol -o BurnableCrowdsaleToken.html
```

would produce file `BurnableCrowdsaleToken.html` with the full report

#### Standalone docker images

The docker images in this repository can be independently summoned to use the available tools without `solsa`. They assume access to a directory with all the required contracts and metadata at `/src`, which would make invoking solium, for example, look like this:

```
$ docker run -it --rm -v $(pwd):/src:ro enhancedsociety/solium -f contracts/UpgradeableToken.sol

contracts/UpgradeableToken.sol
  53:2      error      No visibility specified explicitly for UpgradeableToken function.    security/enforce-explicit-visibility
  65:8      error      Consider using 'revert()' in place of deprecated 'throw'.            security/no-throw
  69:22     error      Consider using 'revert()' in place of deprecated 'throw'.            security/no-throw
  79:6      warning    Use emit statements for triggering events.                           emit
  89:8      error      Consider using 'revert()' in place of deprecated 'throw'.            security/no-throw
  92:24     error      Consider using 'revert()' in place of deprecated 'throw'.            security/no-throw
  94:39     error      Consider using 'revert()' in place of deprecated 'throw'.            security/no-throw
  96:55     error      Consider using 'revert()' in place of deprecated 'throw'.            security/no-throw
  101:41    error      Consider using 'revert()' in place of deprecated 'throw'.            security/no-throw
  103:57    error      Consider using 'revert()' in place of deprecated 'throw'.            security/no-throw
  105:6     warning    Use emit statements for triggering events.                           emit
  111:36    warning    Use 'view' instead of deprecated 'constant'.                         no-constant
  124:25    error      Consider using 'revert()' in place of deprecated 'throw'.            security/no-throw
  125:39    error      Consider using 'revert()' in place of deprecated 'throw'.            security/no-throw
  132:31    warning    Use 'view' instead of deprecated 'constant'.                         no-constant

✖ 11 errors, 4 warnings found.

```

for ease of use you can set up the following alias (drop it in your `.bashrc` or equivalent)

```sh
function docker-run-here () { docker run -it --rm -v $(pwd):/src:ro $@ }
```

which would turn the initial command into

```sh
docker-run-here enhancedsociety/solium -f contracts/UpgradeableToken.sol
```


## TODO

  - [ ] Improve README's [Usage](#Usage) section with example screenshots/asciinema casts
  - [ ] Reintroduce echidna
  - [ ] Add solgraph
  - [ ] Add tests
  - [ ] Reintroduce MAIAN (wait for upstream/port to py3) or give up on it altogether
  - [ ] [NEVERENDING] keep finding, evaluating and integrating tools to improve quality of contracts developed
