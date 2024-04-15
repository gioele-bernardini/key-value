#!/bin/bash
# Write a shell script that accepts a number n greater than 0 from command line. 
# and prints all the value n of the following series Si
# S0 = 0, 
# Si = 2*i + i^2 + Si-1, if i is even (2, 4, 6, 8 ...)
# Si = -2*i + i^2 + Si-1, if i is odd (1, 3, 5, 7 ...)

if [ $1 -lt 1 ]; then
	echo "Number must be greater or equal than 1"
	exit
fi

#initialize vars
s_old=0
s_new=0

for ((i = 1; i <= $1; i++)); do
	is_odd=$(($i%2))
	if [ $is_odd -eq 1 ]; then
		# is is odd
		s_new=$((2*$i + $i*$i + s_old))
	else
		# is is even
		s_new=$((-2*$i + $i*$i + s_old))
	fi
	s_old=$s_new	
done
echo $s_new
