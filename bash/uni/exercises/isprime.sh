#!/bin/bash
# Write a shell script that accepts a number input by user and check if it is prime or not. 
# Using function is must here

read -p "Enter number: " num

isprime () {
	for((i=2; i<=$num/2; i++)); do
		rest=$(($num % $i)) 
		if [ $rest -eq 0 ]; then
			echo "$num is not a prime number."
			exit
		fi
	done
	echo "$num is prime number."
}

isprime $num
