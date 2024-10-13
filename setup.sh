#!/bin/bash

# Install dependencies
npm i -g quicktype

# run transpiler
quicktype --src schema.json \
--src-lang json \
--lang rust \
--out workspace/gh-workflow-rs/src/model.rs

# derive Clone and Setters in model.rs

python3 deriveclone.py