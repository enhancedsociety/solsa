#! /bin/bash

set -eu

################################################################################
#
#    Runs all static analysis bundled with
#    the container on the given contract.
#    Fails with proper exit code so it
#    can be integrated on CI or scripts.
#
################################################################################
ALL_TOOLS="solc,oyente,solium,mythril,echidna,maian"
TOOLS=()

while getopts ":hat:" opt; do
    case $opt in
        h)
            echo "Usage:"
            echo "    $0 -h"
            echo "                           Display this help message"
            echo "    $0 -a CONTRACT_PATH"
            echo "                           Run all tools"
            echo "    $0 -t TOOL [-t TOOL] CONTRACT_PATH"
            echo "                           Run selected tools (out of ${ALL_TOOLS})"
            exit 0
            ;;
        a)
            set -f                      # avoid globbing (expansion of *).
            # shellcheck disable=SC2206
            TOOLS=(${ALL_TOOLS//,/ })
            set +f
            ;;
        t)
            if [[ ! $OPTARG =~ , && $ALL_TOOLS =~ (^|,)${OPTARG}(,|$) ]]; then
                TOOLS+=("${OPTARG}")
            else
                echo "Invalid tool: $OPTARG" >&2
            fi
            ;;
        \?)
            echo "Invalid option: -$OPTARG" >&2
            exit 1
            ;;
        :)
            echo "Option -$OPTARG requires an argument." >&2
            exit 1
            ;;
    esac
done
shift $(( OPTIND - 1 ))

if [ ${#TOOLS[@]} -eq 0 ]; then
    echo "No tools selected. Usage:"
    echo "    $0 -h"
    echo "                           Display this help message"
    echo "    $0 -a CONTRACT_PATH"
    echo "                           Run all tools"
    echo "    $0 -t TOOL [-t TOOL] CONTRACT_PATH"
    echo "                           Run selected tools (out of ${ALL_TOOLS})"
    exit 1
fi

## copy read-only src to location where tools may need write permissions

cp -ar /src /proj

## change to working dir

cd /proj || exit 1

## compile contract
if [[ " ${TOOLS[*]} " =~ " solc " ]]; then
    echo "SOLC"
    solc --bin --abi --metadata --allow-paths . "$1"
fi

## oyente
if [[ " ${TOOLS[*]} " =~ " oyente " ]]; then
    echo "OYENTE"
    python /usr/local/lib/python2.7/dist-packages/oyente/oyente.py --global-timeout 300 --timeout 10000 -a -s "$1"
fi

## solium
if [[ " ${TOOLS[*]} " =~ " solium " ]]; then
    echo "SOLIUM"
    solium -c /etc/.soliumrc.json -f "$1"
fi

## mythril
if [[ " ${TOOLS[*]} " =~ " mythril " ]]; then
    echo "MYTHRIL"
    myth -x "$1"
fi

## echidna
if [[ " ${TOOLS[*]} " =~ " echidna " ]]; then
    echo "ECHIDNA"
    /root/.local/bin/echidna-test "$1"
fi

## maian
if [[ " ${TOOLS[*]} " =~ " maian " ]]; then
    echo "MAIAN"

    solc --bin --abi --allow-paths . -o /opt/MAIAN/tool/out/ "$1"
    cd /opt/MAIAN/tool

    for contract in out/*.bin
    do
        python maian.py -bs "$contract" -c 0
        python maian.py -bs "$contract" -c 1
        python maian.py -bs "$contract" -c 2
    done
fi