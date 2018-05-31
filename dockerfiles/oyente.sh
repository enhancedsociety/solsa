#! /bin/bash -e

cp -a /src /out

cd /out

python /usr/local/lib/python3.6/site-packages/oyente/oyente.py $@