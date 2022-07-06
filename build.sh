#!/bin/bash
# https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/
set -euxo pipefail

cargo zigbuild --target x86_64-unknown-linux-gnu.2.26 --release
mkdir -p functions
cp target/x86_64-unknown-linux-gnu/release/hello-world functions/
cp target/x86_64-unknown-linux-gnu/release/calculator functions/
cp target/x86_64-unknown-linux-gnu/release/json-response functions/
cp target/x86_64-unknown-linux-gnu/release/querystring functions/
cp target/x86_64-unknown-linux-gnu/release/redirect functions/
cp target/x86_64-unknown-linux-gnu/release/reqwest-dad-joke functions/

