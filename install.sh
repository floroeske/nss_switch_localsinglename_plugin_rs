#!/bin/bash
set -eux

cargo build --release

pushd $(dirname $0)/target/release
cp libnss_localsinglename.so libnss_localsinglename.so.2
sudo install -m 0644 libnss_localsinglename.so.2 /lib
sudo /sbin/ldconfig -n /lib /usr/lib

