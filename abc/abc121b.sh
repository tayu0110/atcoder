#!/bin/bash
read n m c
read -a b
ans=0
m=$(($m-1))
for i in $(seq 1 $n); do
  read -a a
  sum=0
  for j in $(seq 0 $m); do
    sum=$((${a[j]}*${b[j]}+sum))
  done
  if [ $(($sum+$c)) -gt 0 ]; then
    ans=$(($ans+1))
  fi
done
echo $ans