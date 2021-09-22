#!/bin/bash
set -e

cargo test lib::tests::increment -- --exact
cargo test lib::tests::decrement -- --exact
cargo test lib::tests::increment_and_reset -- --exact


