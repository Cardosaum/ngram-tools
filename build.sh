#!/bin/bash

echo "Building tally-ngrams..."
cc -g -Wall -pedantic -Wextra -Wno-missing-field-initializers \
    -I./argparse -I./tkvdb \
    argparse/argparse.c \
    tkvdb/tkvdb.c \
    common.c \
    ngrams.c \
    readall.c \
    tally-ngrams.c \
    -o tally-ngrams

echo "Building text-to-ngrams..."
cc -g -Wall -pedantic -Wextra -Wno-missing-field-initializers \
    -I./argparse \
    argparse/argparse.c \
    common.c \
    ngrams.c \
    readall.c \
    text-to-ngrams.c \
    -o text-to-ngrams

echo "Done."