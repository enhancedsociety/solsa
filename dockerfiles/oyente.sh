#! /bin/bash -e

cp -rs /src /out

cd /out

python /usr/local/lib/python3.6/site-packages/oyente/oyente.py $@