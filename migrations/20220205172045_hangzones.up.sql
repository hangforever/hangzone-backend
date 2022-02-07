-- Add up migration script here
CREATE TABLE hangzones (
  id serial PRIMARY KEY,
  slug text NOT NULL,
  name text NOT NULL,
  description text,
  address_1 text NOT NULL,
  address_2 text,
  address_3 text,
  city text NOT NULL,
  state text,
  country text NOT NULL,
  postal_code varchar(20) NOT NULL,
  geography geography(point) NOT NULL,
  created_at timestamp NOT NULL,
  updated_at timestamp NOT NULL
);