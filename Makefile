#ToDo add disel stuff to run auto up down
CARGO=cargo

DIR_SCRIPTS=./scripts

.PHONY: all
all: fmt test run

run:
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

watch:
	$(CARGO) $@ -x run

prepare: db-prepare db-setup

db-prepare:
	$(DIR_SCRIPTS)/docker_compose.sh

db-setup:
	diesel setup