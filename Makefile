CARGO		:= cargo
SCRIPT_DIR	:= ./scripts

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
	$(SCRIPT_DIR)/docker_compose.sh

.PHONY: db-setup
db-setup:
	diesel setup