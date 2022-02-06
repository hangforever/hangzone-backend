#!/bin/bash

for migration in $(ls /app/migrations)
do
  if [[ "$migration" == *"up.sql"* ]]; then
    psql -U $POSTGRES_USER -d $POSTGRES_DB -a -f /app/migrations/$migration
  fi
done