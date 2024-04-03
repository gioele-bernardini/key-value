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
case $ch in
  1)
  res=$(($a + $b))
  ;;
  2)
  let res=$a-$b
  ;;
  3)
  res=`expr $a \* $b`
  ;;
  4)
  res=$(expr $a / $b)
  ;;
esac
echo "Result : $res"
