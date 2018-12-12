#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && cd ../ > /dev/null && pwd )"

pushd $DIR

#watches for changes in ./src and reruns tests
while inotifywait -e close_write ./src/ -r; do cargo test; done

popd

