#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null  && cd ../ > /dev/null && pwd )"

pushd $DIR >/dev/null
source ./venv/bin/activate
python ./reddit.py
python ./twitter_pop.py
popd >/dev/null
