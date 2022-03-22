for seed in (ls ./seeds/); psql -d hangzone -a -f ./seeds/$seed; end
