#!/bin/bash
# Write a shell script that takes two parameters and behaves as follows:
# if both are numbers, it sums them and prints result
# if both are strings, it concats them and prints result
# if the first is number and other is not, it outputs the first parameter
# if the first is string and the second is number, it checks if the string starts with a number: if yes, it echoes "OS".
# Tip: you can check if a string contains a number by using regexes and the =~ notation

if [[ $1 =~ ^[0-9]+$ ]] && [[ $2 =~ ^[0-9]+$ ]]; then 
	sum=$(($1+$2))
	echo "exercising sum: ${sum}"
elif [[ ! $1 =~ ^[0-9]+$ ]] && [[ ! $2 =~ ^[0-9]+$ ]]; then 
	echo "concat: ${1}${2}"
elif [[ ! $1 =~ ^[0-9]+$ ]] && [[ $2 =~ ^[0-9]+$ ]]; then 
	if [[ $1 =~ ^[0-9] ]]; then
		echo "OS"
	fi
elif [[ $1 =~ ^[0-9]+$ ]] && [[ ! $2 =~ ^[0-9]+$ ]]; then 
	echo "first param: ${1}"
else
	echo "you should never get here..."
fi



