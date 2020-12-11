.PHONY: build

CRATES = $(dir $(wildcard ./crates/*/))

build:
	@echo $(CRATES)
	for crate in $(CRATES) ; do \
		cargo install --path $$crate --root bins --target x86_64-unknown-linux-musl ;\
	done
	@mkdir -p functions
	@cp bins/bin/* functions/