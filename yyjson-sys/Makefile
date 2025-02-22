.SILENT:
.DEFAULT_GOAL:=run
SHELL:=/usr/bin/bash

.PHONY: run clean

run:
	echo run

bindgen: src/bindings.rs

src/bindings.rs: yyjson/src/yyjson.h
	bindgen $< -o $@

clean:
	echo clean
