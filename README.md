# Smart contracts advanced CI pipeline

The goal of this repo is to find a good way of integrating static analisys tools for smart contracts into CI pipelines.
Ideally, this should be be easy to pick up and use by any project using smart contracts (dapps, wallets, etc).

The main challenges to overcome appear to be:
    - seamless installation and configuration of all tools
    - invocation of each tools taking into account its specificities (sometimes on things as simple as where contracts need to be located)
    - resolution of contract dependencies!!! (biggest identified PITA so far)


#### WARNING

This repo may not be exactly in a working state!!!

## Key tools for linting and static analisys

    - solc - compile (portable)
    - Solium - lint (depends on npm)
    - Oyente - static analisys (depends on pip)
    - MAIAN - static analisys (depends on pip, must be built from source along with its dependencies)
    - mythril - static analisys (depends on pip)

## Tools in need of experimentation/validation

    - https://github.com/trailofbits/echidna (some trouble installing so far)