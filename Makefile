CDIR=src/c
CC=clang
BUILD_DIR=target/release
INCLUDE_DIR=src/include
LD=$(BUILD_DIR)
RUST_TARGET=--release

all: rust c

debug: RUST_TARGET=
debug: C_DEBUG=-g -O0
debug: BUILD_DIR=target/debug
debug: rust c

libisaac: rust

rust:
	INCLUDE_DIR=$(INCLUDE_DIR) cargo build $(RUST_TARGET)

c: libisaac
	$(CC) -I$(INCLUDE_DIR) $(CDIR)/main.c -L$(LD) -lisaac $(C_DEBUG) -o $(BUILD_DIR)/cisaac

run: c
	LD_LIBRARY_PATH=$(LD) $(BUILD_DIR)/cisaac
