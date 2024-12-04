#!/bin/bash

while true; do
	inotifywait -e close_write src/*
	cargo test
done
