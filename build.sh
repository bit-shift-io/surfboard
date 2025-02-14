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
    
    tools
    ===================
    m) Modify SVG's

    *) Any key to exit
    :" ans;
    reset
    case $ans in
        1) fn_run ;;
        2) fn_build ;;
        3) fn_build_debug ;;
        4) fn_run_debug ;;
        m) fn_mod_svg ;;
        *) $SHELL ;;
    esac
    done
}


function fn_mod_svg {
    for file in res/*.svg; do
        echo $'\n'"$file"
        
        # remove comments
        sed -i 's/<!--.*-->//g' "$file"
        
        # remove new lines space etc between tags
        sed -i 's/>[[:space:]]*</></g' "$file"
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
