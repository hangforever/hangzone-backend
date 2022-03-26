#!/bin/bash

while [ true ]
do
  if [[ $(sqlx migrate revert) =~ 'No migrations' ]]; then
    break
  fi
done
