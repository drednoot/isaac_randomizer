CDIR=src/c
CC=clang
BUILD_DIR=target/release
INCLUDE_DIR=src/include
LD=$(BUILD_DIR)

all: rust c

rust:
	INCLUDE_DIR=$(INCLUDE_DIR) cargo build --release

c: rust
	$(CC) -I$(INCLUDE_DIR) $(CDIR)/main.c -L$(LD) -lisaac -o $(BUILD_DIR)/cisaac

run: c
	LD_LIBRARY_PATH=$(LD) $(BUILD_DIR)/cisaac
