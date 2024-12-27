.DEFAULT_GOAL := all

.PHONY: all run test

all:
	for day in day*; do \
		cargo run --release --quiet --bin=$$day ; \
	done

run:
	cargo run --release --quiet --bin=day$(day)

test:
	cargo test
