CARGO=cargo

DIR_SCRIPTS=./scripts

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

prepare:
	$(DIR_SCRIPTS)/docker_compose.sh