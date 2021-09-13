.PHONY: build

CRATES = $(dir $(wildcard ./crates/*/))
CRATES_2 = $(bash -c "fd . --base-directory ./target/x86_64-unknown-linux-musl/release/ -d1 -tx")
CRATES_3 = $(dir $(wildcard ./crates/*/))
CRATE_NAMES := $(shell fd . --base-directory ./target/x86_64-unknown-linux-musl/release/ -d1 -tx)

build:
	@echo $(CRATES)
	for crate in $(CRATES) ; do \
		RUSTFLAGS='-C link-arg=-s' cargo install --path $$crate --root bins --target x86_64-unknown-linux-musl ;\
	done
	mkdir -p functions
	cp bins/bin/* functions/
	ls functions

local:
	cargo build --release --target x86_64-unknown-linux-musl
	fd . --base-directory ./target/x86_64-unknown-linux-musl/release/ -d1 -tx | xargs -n1 printf "target/x86_64-unknown-linux-musl/release/%s functions/ " | xargs -n2 cp