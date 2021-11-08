#!/bin/bash
read n
read -a x
ans=1000000
for ((i = 0; i < 110; i++)); do
  tmp=0
  for ((j = 0; j < n; j++)); do
    tmp=$((tmp + (x[j] - i) * (x[j] - i)))
  done
  if ((tmp < ans)); then
    ans=$tmp
  fi
done
echo $ans