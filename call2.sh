#!/bin/bash
set -e

near call $ACCOUNT_ID --accountId=$ACCOUNT_ID2 --gas=300000000000000 "$@"
