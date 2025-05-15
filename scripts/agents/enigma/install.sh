#!/bin/bash
set -eu

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# pull the enigma image
docker pull sweagent/enigma:latest

# clone the repo
ENIGMA_REPO="$SCRIPT_DIR/enigma-repo"
# git clone https://github.com/SWE-agent/SWE-agent $ENIGMA_REPO && \
#     cd $ENIGMA_REPO && \
#     git checkout 34f55c7bb14316193cdfee4fd5568928c7b65f60 # v0.7.0
git clone -b cybergym git@github.com:sunblaze-ucb/cybergym-enigma.git $ENIGMA_REPO


echo "Install the dependencies in your python venv with \`pip install -r requirements.txt\`"