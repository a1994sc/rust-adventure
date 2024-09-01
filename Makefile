.PHONY: all
all: build

.PHONY: build
build:
	@nix build .#rust-testing

.PHONY: clean
clean:
	@rm -rf result .direnv target

.PHONY: test
test:
	@cargo test

.PHONY: fmt
fmt:
	@nix fmt

.PHONY: update-flake
update-flake:
	@nix flake update

.PHONY: update-cargo
update-cargo:
	@cargo update
