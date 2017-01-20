CC=gcc
CFLAGS=-O2 -Wall -std=c11
BUILD_DIR=build

SRC=$(wildcard src/problem-*.c)
PROBLEMS=$(patsubst %.c,%,$(SRC))

all: $(BUILD_DIR) $(PROBLEMS)

problem-% : problem-%.c
	$(CC) $(CFLAGS) $< -o $(BUILD_DIR)/$(@F)

$(BUILD_DIR):
	mkdir $(BUILD_DIR)

.PHONY: clean
clean:
	rm -r $(BUILD_DIR)
