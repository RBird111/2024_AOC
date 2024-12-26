.DEFAULT_GOAL := all

.PHONY: all run

all:
	for day in day*; do \
		cargo run -rq --bin=$$day ; \
	done

run:
	cargo run -rq --bin=day$(day)
