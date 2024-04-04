#!/usr/bin/env bash

if [[ $# -ne 1 ]]; then
  # doppie apici per inserire argomenti!
  echo "Usage: $0 <number>"
  exit 1
fi

# if [[ $1 =~ ^[a-z|A-Z]+$ ]]; then
if [[ $1 =~ ^[[:alpha:]]+$ ]]; then
  echo 'The argument contains only letters'
  exit 0
else
  echo 'The argument does not contains only letters'
fi