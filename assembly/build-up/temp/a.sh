#!/usr/bin/env bash

if [ $# -ne 1 ]; then
  echo "Usage: $0 <file.asm>"
  exit 1
fi

filename=$(basename -- "$1")
filename_no_ext="${filename%.*}"

nasm -f elf64 "$filename" -o "${filename_no_ext}.o"
ld "${filename_no_ext}.o" -o "$filename_no_ext"
