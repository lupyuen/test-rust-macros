#!/usr/bin/env bash

set -e  #  Exit when any command fails.
set -x  #  Echo all commands.

rustup default nightly
cargo rustc -- -Z unstable-options --pretty expanded > log/main-expanded.rs
