.PHONY: build

CRATES = $(dir $(wildcard ./crates/*/))

build:
	@echo $(CRATES)
	for crate in $(CRATES) ; do \
		RUSTFLAGS='-C link-arg=-s' cargo install --path $$crate --root bins --target x86_64-unknown-linux-musl ;\
	done
	mkdir -p functions
	cp bins/bin/* functions/
	ls functions