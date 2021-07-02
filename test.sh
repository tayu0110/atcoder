#!/bin/bash

echo Shellscript test
t="test"
fnumber=3
temp="33.0C"
loc="Tokyo"
date=`date`
echo $t,"Ethernet1/"$fnumber,$temp,$loc,$date > test.txt
