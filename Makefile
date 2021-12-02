PROJECT_NAME = 'rust_bevy_tile_example'

# Version
VERSION = `date +%y.%m`

# If unable to grab the version, default to N/A
ifndef VERSION
    VERSION = "n/a"
endif

FLAGS =
LLD := $(shell which lld)

ifeq ($(LLD), /usr/bin/lld)
	FLAGS = -C link-arg=-fuse-ld=lld
endif

#
# Makefile options
#


# State the "phony" targets
.PHONY: all debug build run clean

all: build

debug:
	@echo 'Building debug binary...'
	@cargo build

build:
	@echo 'Building release...'
	@cargo build --release

run:
	@RUSTFLAGS="${FLAGS}" cargo run --release

lint:
	@cargo clippy

clean:
	@echo 'Cleaning...'
	@cargo clean
