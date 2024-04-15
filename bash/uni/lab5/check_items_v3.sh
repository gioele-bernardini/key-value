#!/bin/bash
# Write a shell script that checks if a folder contains more files (no directories) than its parent (non-recursive).
# If yes, it outputs the number of directory in the current folder, 
# otherwise it outputs the number of directories in the parent folder

# Tip: remember that the parent folder can be accessed via the ".." naming in paths
# Tip: you can use the maxdepth param of find for limiting the find to a single folder (find . -maxdepth 1 -type f)

if [ $# -eq 0 ]; then
	echo "no folder specified"
	exit 1
fi

cd $1

files_folder=$(find . -maxdepth 1 -type f | wc -l)
files_parent=$(find .. -maxdepth 1 -type f | wc -l)

out_var=0
if [ $files_folder -gt $files_parent ] ; then
	echo "there are more files in the specified directory: $files_folder vs $files_parent"
	out_var=$(find . -type d | wc -l)		
else
	echo "there are more files in the parent of the specified directory: $files_folder vs $files_parent"
	out_var=$(find .. -type d | wc -l)
fi

echo "directories: $out_var"
