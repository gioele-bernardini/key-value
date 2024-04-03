#!/usr/bin/env bash

foo() {

	mkdir "$1"
	cd "$1"
	
}

foo "$1"
