#!/bin/bash
sqlx migrate run

for seed in $(ls ./seeds)
do
  psql -U $POSTGRES_USER -d $POSTGRES_DB -a -f "./seeds/$seed"
done
