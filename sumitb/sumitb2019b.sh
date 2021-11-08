#!/bin/bash
read n
t=$(($n * 100 / 108 - 10))
for ((ans=$t; $ans < $t+20; ans++)) {
  k=$(($ans * 108 / 100))
  if [ $k -eq $n ]; then
    echo $ans
    exit
  fi
}
echo ":("