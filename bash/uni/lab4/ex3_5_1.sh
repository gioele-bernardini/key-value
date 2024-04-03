#!/usr/bin/env bash

time=$(date +%H)

if [ $time -gt 5 ] && [ $time -lt 12 ]; then
	echo "Good morning!"
elif [ $time -gt 12 ] && [ $time -lt 18 ]; then
	echo "Good evening!"
elif [ $time -gt 18 ] && [ $time -lt 5 ]; then
	echo "Good night!"
fi