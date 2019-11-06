#!/usr/bin/env bash

source ~/.bash_profile
dir=`pwd`

NDK_STANDALONE=$HOME/ndk-standalone
export PATH="$PATH":"$NDK_STANDALONE/arm64/bin"
export PATH="$PATH":"$NDK_STANDALONE/arm/bin"
export PATH="$PATH":"$NDK_STANDALONE/x86/bin"

cd rust-lib \
&& RUST_BACKTRACE=1 cargo +nightly build --target i686-linux-android \
&& RUST_BACKTRACE=1 cargo +nightly build --target armv7-linux-androideabi
