#!/usr/bin/env bash

COMPILATOR="/tmp/target/debug/l-compilator"
ASSEMBLER="./assemble.sh"

mkdir x86

for l_file in testor/*.l
do
  file="$(basename $l_file .l)"
  $COMPILATOR -n $l_file > x86/$file.nasm
  $ASSEMBLER x86/$file.nasm

  in_file=""
  if [ -f "testor/$file.in" ]
  then
      in_file="$(cat testor/$file.in)"
  fi

  x86/$file <<< "$in_file" > x86/$file.out

  echo -en "\e[96m$file \e[0m"
  if [ "$(cat testor/$file.out)" == "$(cat x86/$file.out)" ]
    then
        echo -e "\e[92mok\e[0m"
    else
        echo -e "\e[91merror\e[0m"
        echo "get:"
        cat x86/$file.out
        echo "expected:"
        cat testor/$file.out
  fi
done
