#!/bin/bash

for i in {a..e}
do
	cp temp.cpp abc$1$i.cpp
	code abc$1$i.cpp
done
