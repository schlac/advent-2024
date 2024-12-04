#!/bin/bash

pushd "$1"
while true; do
	inotifywait -e close_write Cargo.toml src/*.rs
	cargo test
done
popd
