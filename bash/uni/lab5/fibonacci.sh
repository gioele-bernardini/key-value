#!/bin/bash
# Write a shell script that accepts a number greater than 2 from command line and prints the Fibonacci sequence up to that term.


if [ $1 -lt 2 ]; then
	echo "Number must be greater than 2"
	exit
fi

#initialize first and second terms
t1=0
t2=1

echo -n "$t1, $t2"

for ((i = 2; i <= $1; ++i)); do
	nextTerm=$(($t1 + $t2));
	echo -n ", $nextTerm";
	t1=$t2;
	t2=$nextTerm;	
done

echo ""
