SHELL := /bin/bash

VERSION=$(shell cat VERSION)

ifeq ($(shell uname -s),Darwin)
CONFIG_DARWIN=y
else ifeq ($(OS),Windows_NT)
CONFIG_WINDOWS=y
else
CONFIG_LINUX=y
endif

LIBRARY_PREFIX=lib
ifdef CONFIG_DARWIN
LOADABLE_EXTENSION=dylib
STATIC_EXTENSION=a
endif

ifdef CONFIG_LINUX
LOADABLE_EXTENSION=so
STATIC_EXTENSION=a
endif


ifdef CONFIG_WINDOWS
LOADABLE_EXTENSION=dll
LIBRARY_PREFIX=
STATIC_EXTENSION=lib
endif

ifdef target
CARGO_TARGET=--target=$(target)
BUILT_LOCATION=target/$(target)/debug/$(LIBRARY_PREFIX)sqlite_jiff.$(LOADABLE_EXTENSION)
BUILT_LOCATION_RELEASE=target/$(target)/release/$(LIBRARY_PREFIX)sqlite_jiff.$(LOADABLE_EXTENSION)
BUILT_LOCATION_STATIC=target/$(target)/debug/$(LIBRARY_PREFIX)sqlite_jiff.$(STATIC_EXTENSION)
BUILT_LOCATION_STATIC_RELEASE=target/$(target)/release/$(LIBRARY_PREFIX)sqlite_jiff.$(STATIC_EXTENSION)
else
CARGO_TARGET=
BUILT_LOCATION=target/debug/$(LIBRARY_PREFIX)sqlite_jiff.$(LOADABLE_EXTENSION)
BUILT_LOCATION_RELEASE=target/release/$(LIBRARY_PREFIX)sqlite_jiff.$(LOADABLE_EXTENSION)
BUILT_LOCATION_STATIC=target/debug/$(LIBRARY_PREFIX)sqlite_jiff.$(STATIC_EXTENSION)
BUILT_LOCATION_STATIC_RELEASE=target/release/$(LIBRARY_PREFIX)sqlite_jiff.$(STATIC_EXTENSION)
endif


ifdef python
PYTHON=$(python)
else
PYTHON=python3
endif

prefix=dist
TARGET_LOADABLE=$(prefix)/debug/jiff0.$(LOADABLE_EXTENSION)
TARGET_LOADABLE_RELEASE=$(prefix)/release/jiff0.$(LOADABLE_EXTENSION)

TARGET_STATIC=$(prefix)/debug/jiff.a
TARGET_STATIC_RELEASE=$(prefix)/release/jiff0.a


ifdef target
CARGO_TARGET=--target=$(target)
BUILT_LOCATION=target/$(target)/debug/$(LIBRARY_PREFIX)sqlite_jiff.$(LOADABLE_EXTENSION)
BUILT_LOCATION_RELEASE=target/$(target)/release/$(LIBRARY_PREFIX)sqlite_jiff.$(LOADABLE_EXTENSION)
else
CARGO_TARGET=
BUILT_LOCATION=target/debug/$(LIBRARY_PREFIX)sqlite_jiff.$(LOADABLE_EXTENSION)
BUILT_LOCATION_RELEASE=target/release/$(LIBRARY_PREFIX)sqlite_jiff.$(LOADABLE_EXTENSION)
endif


$(prefix):
	mkdir -p $(prefix)/debug
	mkdir -p $(prefix)/release


$(TARGET_LOADABLE): $(prefix) $(shell find . -type f -name '*.rs')
	cargo build $(CARGO_TARGET)
	cp $(BUILT_LOCATION) $@

$(TARGET_LOADABLE_RELEASE): $(prefix) $(shell find . -type f -name '*.rs')
	cargo build --release $(CARGO_TARGET)
	cp $(BUILT_LOCATION_RELEASE) $@

Cargo.toml: VERSION
	cargo set-version `cat VERSION`


version:
	make Cargo.toml

format:
	cargo fmt


release: $(TARGET_LOADABLE_RELEASE) $(TARGET_STATIC_RELEASE)

loadable: $(TARGET_LOADABLE)
loadable-release: $(TARGET_LOADABLE_RELEASE)

static: $(TARGET_STATIC) $(TARGET_H)
static-release: $(TARGET_STATIC_RELEASE) $(TARGET_H_RELEASE)

debug: loadable static python datasette
release: loadable-release static-release python-release datasette-release

clean:
	rm -rf dist
	cargo clean

SOLITE=solite

test-snap:
	TZ=America/Los_Angeles $(SOLITE) snap tests/snapshot.sql

.PHONY: test-snap

publish-release:
	./scripts/publish_release.sh

.PHONY: clean \
	test test-loadable \
	loadable loadable-release \
	static static-release \
	debug release \
	format version publish-release
