#!/bin/bash

for i in {a..c}
do
	cp temp.cpp agc$1$i.cpp
	code agc$1$i.cpp
done
