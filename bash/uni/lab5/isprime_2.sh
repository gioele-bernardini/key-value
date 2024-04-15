#!/bin/bash
# Write a shell script that accepts a number from commandline and check if it is prime or not. 
# Using function is must here

if [ $# -ne 1 ] ; then
	echo "must provide a parameter"
	exit
fi

isprime () {
	for((i=2; i<=$1/2; i++)); do
		rest=$(($1 % $i)) 
		if [ $rest -eq 0 ]; then
			echo "$1 is not a prime number."
			exit
		fi
	done
	echo "$1 is prime number."
}

isprime $1
