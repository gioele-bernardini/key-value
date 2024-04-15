#!/bin/bash
# Write a shell script toconvert a temperature from C to F and from F to C degrees
# Ctemp = (Ftemp-32)*1.8
# Ftemp = Ctemp*1.8+32
# the script should take two inputs: the number and a letter, either F or C, which indicates if the number is celsius of farenheit
# the script should output the temperature in the other measure unit.
# The student should implement two functions that take a number each, one for C to F and one for F to C 
# Tip: you cant directly edit the $1, $2, ... function vars, you have to copy them into another variable first

C_to_F()
{
	tmp=$1
	let tmp=$tmp*18/10
	let tmp=$tmp+32
	echo "C temperature is $1, F temperature is $tmp"
}

F_to_C()
{
	tmp=$1
	let tmp=$tmp-32
	let tmp=$tmp/18*10
	echo "F temperature is $1, C temperature is $tmp"
}

if [ "$2" == "F" ]
then
	F_to_C $1

elif [ "$2" == "C" ]
then
	C_to_F $1
else
	echo "Unknown unit"
fi

