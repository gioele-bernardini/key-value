#!/usr/bin/env bash

say_hi(){

	if [ $1 -gt 5 ] && [ $1 -lt 12 ]; then
		echo "Good morning!"
	elif [ $1 -gt 12 ] && [ $1 -lt 18 ]; then
		echo "Good evening!"
	elif [ $1 -gt 18 ] && [ $1 -lt 5 ]; then
		echo "Good night!"
	fi
}

time=$(date +%H)
say_hi $time

