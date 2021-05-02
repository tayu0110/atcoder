#!/bin/bash

for i in {a..d}
do
	cp temp.cpp arc$1$i.cpp
	code arc$1$i.cpp
done
