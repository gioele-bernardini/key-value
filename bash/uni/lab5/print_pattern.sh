#!/bin/bash
# Write a shell script that accepts a parameter n, checks if it is a number and if yes prints n lines with the pattern
# x
# xx
# xxx
# xxxx
# .......
# xxxxxxxxxxx (n times)
# Tip: remember that you can concatenate strings as var="${var}x"

if [ $# -ne 1 ] || [[ ! $1 =~ ^[0-9]+$ ]]; then
	echo "param should exist and be int"
	exit
fi

for ((i = 1; i <= $1; i++)); do
	rowPattern=""
	for ((j = 1; j <= i; j++)); do
		rowPattern="${rowPattern}x"
	done
	
	echo $rowPattern	
done

echo ""
