-- Add up migration script here
CREATE TABLE user_hangers (
  id serial PRIMARY KEY,
  first_name varchar(120) NOT NULL,
  last_name varchar(120) NOT NULL,
  alias varchar(120) NOT NULL,
  email_address varchar(320),
  status_hang integer NOT NULL,
  status_description text,
  icon_url text,
  hash text NOT NULL,
  geography geography(point),
  created_at timestamptz NOT NULL DEFAULT NOW(),
  updated_at timestamptz NOT NULL DEFAULT NOW()
);

CREATE TABLE friends (
  id serial PRIMARY KEY,
  user_hanger_id integer REFERENCES user_hangers NOT NULL,
  friend_user_hanger_id integer REFERENCES user_hangers NOT NULL,
  created_at timestamptz NOT NULL DEFAULT NOW(),
  updated_at timestamptz NOT NULL DEFAULT NOW()
);

