#!/bin/bash

function main {
    # loop args
    if [[ $# -ne 0 ]] ; then
        for var in "$@" ; do
            eval $var
        done
        exit 1
    fi
    
    # menu
    while true; do
    read -n 1 -p "
    config
    ===================
    1) Run Release
    2) Build Release
    
    3) Build Debug
    4) Run Debug

    *) Any key to exit
    :" ans;
    reset
    case $ans in
        1) fn_run ;;
        2) fn_build ;;
        3) fn_build_debug ;;
        4) fn_run_debug ;;
        *) $SHELL ;;
    esac
    done
}


function fn_run {
    cp ./target/release/surfboard surfboard
    ./surfboard
}


function fn_build {
    cargo build --release
    cp ./target/release/surfboard surfboard
}


function fn_build_debug {
    cargo build
}


function fn_run_debug {
    ./target/debug/surfboard
}


# pass all args
main "$@"
