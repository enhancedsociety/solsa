#! /bin/bash -e


################################################################################
#
#    Runs all static analysis bundled with
#    the container on the given contract.
#    Fails with proper exit code so it
#    can be integrated on CI or scripts.
#
# TODO
#   add option to selectively enable/disable tools
#   add capability of automatically discovering all 
#       contracts within /src and run analysis on 
#       each of them
#   add option to supress output in case of success
#
################################################################################


## copy read-only src to location where tools may need write permissions

cp -ar /src /proj

## change to working dir

cd /proj || exit 1

## compile contract
echo "SOLC"

solc --bin --abi --metadata --allow-paths . "$1"

## oyente
echo "OYENTE"

python /usr/local/lib/python2.7/dist-packages/oyente/oyente.py -a -s "$1"

## solium
echo "SOLIUM"

solium -c /etc/.soliumrc.json -f "$1"

## mythril
echo "MYTHRIL"

myth -x "$1"

## echidna
echo "ECHIDNA"

/root/.local/bin/echidna-test "$1"

## maian
echo "MAIAN"

solc --bin --abi --allow-paths . -o /opt/MAIAN/tool/out/ "$1"

cd /opt/MAIAN/tool

for contract in out/*.bin
do
    python maian.py -bs "$contract" -c 0
    python maian.py -bs "$contract" -c 1
    python maian.py -bs "$contract" -c 2
done