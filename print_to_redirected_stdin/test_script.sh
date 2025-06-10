#!/bin/env bash

set -eux

test_empty_file=$(mktemp)

cargo build

set +e # Temporarily disable err-on-exit.

< ${test_empty_file} cargo run

exit_code=$?

set -e # Re-enable err-on-exit.

rm ${test_empty_file}

exit ${exit_code}
