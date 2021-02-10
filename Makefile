CARGO=cargo

.PHONY: run

run: fmt test
	$(CARGO) $@

build:
	$(CARGO) $@

test:
	$(CARGO) $@

fmt:
	$(CARGO) $@

clean:
	$(CARGO) $@

release:
	$(CARGO) build --release