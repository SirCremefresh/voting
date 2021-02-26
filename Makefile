#ToDo add disel stuff to run auto up down
CARGO=cargo

DIR_SCRIPTS=./scripts

.PHONY: all
all: fmt test run

.PHONY: run
run:
	$(CARGO) $@

.PHONY: build
build:
	$(CARGO) $@

.PHONY: test
test:
	$(CARGO) $@

.PHONY: fmt
fmt:
	$(CARGO) $@

.PHONY: clean
clean:
	$(CARGO) $@

.PHONY: release
release:
	$(CARGO) build --release

.PHONY: watch
watch:
	$(CARGO) $@ -x run

.PHONY: prepare
prepare: db-prepare db-setup

.PHONY: db-prepare
db-prepare:
	$(DIR_SCRIPTS)/docker_compose.sh

.PHONY: db-setup
db-setup:
	diesel setup