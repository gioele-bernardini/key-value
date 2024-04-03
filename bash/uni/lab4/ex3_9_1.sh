#!/bin/bash 
# modified from https://www.geeksforgeeks.org/simple-calculator-bash/
# Take user Input
a=$1
b=$2
 
# Input type of operation
echo "Enter Choice :"
echo "1. Addition"
echo "2. Subtraction"
echo "3. Multiplication"
echo "4. Division"
read ch
 
# Switch Case to perform
# calculator operations
if [ $ch -eq 1 ]; then
  res=$(($a + $b))
elif [ $ch -eq 2 ]; then
  let res=$a-$b
elif [ $ch -eq 3 ]; then
  res=`expr $a \* $b`
elif [ $ch -eq 4 ]; then
  res=$(expr $a / $b)
fi
echo "Result : $res"
