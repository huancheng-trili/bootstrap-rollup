#!/bin/sh

#
mkdir -p ~/.tezos-rollup-node/wasm_2_0_0

cp preimages/* ~/.tezos-rollup-node/wasm_2_0_0/

octez-smart-rollup-node init operator config for sr1Ghq66tYK9y3r8CC1Tf8i8m5nxh8nTvZEf with operators bootstrap2 --boot-sector-file $(pwd)/counter_installer.hex --data-dir ~/.tezos-rollup-node/

octez-smart-rollup-node run operator for "sr1Ghq66tYK9y3r8CC1Tf8i8m5nxh8nTvZEf" \
    with operators "bootstrap2" --data-dir ~/.tezos-rollup-node/ \
    --boot-sector-file $(pwd)/counter_installer.hex
