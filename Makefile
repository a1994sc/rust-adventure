.PHONY: all
all: build

.PHONY: build
build:
	@cargo build --release

.PHONY: clean
clean:
	@rm -rf result .direnv target

.PHONY: test
test:
	@cargo test

.PHONY: update-cargo
update-cargo:
	@cargo update

.PHONY: fmt
fmt:
	@cargo fmt
