#!/bin/bash
# Write a shell script that accepts a string as input, checks if it ends with .sh, if yes prints to screen the contents of the script and executes it. If not, prints "not a script" 
# Tip: use the 'test -f <filename> condition for checking if a file exists

if [ $1=~'.sh$' ]; then
	
	if [ -f './'$1 ]; then
		echo 'calling the script'
		sh './'$1
	fi

fi
