.SILENT:
.DEFAULT_GOAL:=all
SHELL:=/usr/bin/bash

.PHONY: all bindgen

all:
	echo 'yyjson-sys'

BINDINGS_RS = src/bindings.rs
YYJSON_H = yyjson/src/yyjson.h

bindgen:
	bindgen \
		--output $(BINDINGS_RS) \
		--rust-edition 2021 \
		--wrap-static-fns \
		--wrap-static-fns-path extern.c \
		$(YYJSON_H) \
		-- \
		-DYYJSON_DISABLE_NON_STANDARD=1 \
		-DYYJSON_DISABLE_UTILS=1 \
		-DYYJSON_DISABLE_WRITER=1
