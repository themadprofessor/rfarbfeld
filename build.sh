#!/bin/sh
if command -v 2> /dev/null; then
	cargo build --release
	cp target/release/rfarbfeld .
else
	echo "Cargo not found! Please install cargo"
fi
