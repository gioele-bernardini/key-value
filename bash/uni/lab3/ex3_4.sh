#!/bin/bash

make_my_folder() {
	mkdir "$1"
	cd "$1"
	touch "$2"
}

make_my_folder $1 $2
