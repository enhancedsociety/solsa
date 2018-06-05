# Smart contracts tooling aggregator

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

## Install/Run

TODO docs for this as solsa just got completely rewritten.

## Usage

TODO docs for this (and some example screenshots/asciinema casts) as solsa just got completely rewritten.

## TODO

  - [ ] Rewrite README
  - [ ] Reintroduce echidna
  - [ ] Add tests
  - [ ] Suppress output on success (or add quiet option for it)
  - [ ] Reintroduce MAIAN (wait for upstream/port to py3/remove from solsa)
  - [ ] [NEVERENDING] keep finding, evaluating and integrating tools to improve quality of contracts developed
