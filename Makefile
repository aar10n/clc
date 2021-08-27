# this makefile is here to aid with generating the alfred workflow
CLC_KEYWORD = clc # the keyword used to trigger the workflow

.PHONY: build
build:
	cargo build --quiet --release

.PHONY: build-workflow
build-workflow: build
	m4 \
		-DCLC_KEYWORD=$(CLC_KEYWORD) \
		-DCLC_VERSION=$$(grep -o -P '(?<=^version = ")(.*)(?="$$)' Cargo.toml) \
		alfred/info.plist.in > alfred/info.plist
	zip -q -j alfred/clc.alfredworkflow \
		alfred/info.plist \
		alfred/images/* \
		target/release/clc
	rm alfred/info.plist
