.SILENT:
.DEFAULT_GOAL:=all
SHELL:=/usr/bin/bash

.PHONY: all bindgen

all:
	echo 'yyjson-sys'

bindgen: src/bindings.rs

src/bindings.rs: yyjson/src/yyjson.h
	bindgen \
		--output $@ \
		--rust-edition 2021 \
		--wrap-static-fns \
		--wrap-static-fns-path extern.c \
		$<
