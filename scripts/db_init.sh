for migration in $(ls $APP_DIR/migrations)
do
  if [[ "$migration" == *"up.sql"* ]]; then
    psql -U $POSTGRES_USER -d $POSTGRES_DB -a -f $APP_DIR/migrations/$migration
  fi
done
for seed in $(ls $APP_DIR/seeds)
do
  psql -U $POSTGRES_USER -d $POSTGRES_DB -a -f $APP_DIR/seeds/$seed
done
