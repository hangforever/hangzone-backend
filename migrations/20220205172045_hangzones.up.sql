-- Add up migration script here
CREATE TABLE hangzones (
  id serial PRIMARY KEY,
  slug text NOT NULL UNIQUE,
  name text NOT NULL,
  description text,
  address_1 text NOT NULL,
  address_2 text,
  address_3 text,
  city text NOT NULL,
  state text,
  country text NOT NULL,
  postal_code varchar(40),
  geography geography(point) NOT NULL,
  created_at timestamptz NOT NULL DEFAULT NOW(),
  updated_at timestamptz NOT NULL DEFAULT NOW()
);

CREATE TABLE hang_session (
  id serial PRIMARY KEY,
  name text NULL,
  description text,
  hangzone_id integer REFERENCES hangzones,
  starts_at timestamptz NOT NULL DEFAULT NOW(),
);
