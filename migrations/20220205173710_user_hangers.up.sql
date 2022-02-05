-- Add up migration script here
CREATE TABLE user_hangers (
  id serial PRIMARY KEY,
  first_name varchar(40) NOT NULL,
  last_name varchar(40) NOT NULL,
  alias varchar(40) NOT NULL,
  email_address varchar(320),
  status_hang smallint NOT NULL,
  status_description text,
  icon_url text,
  geography geography(point),
  current_hangzone_id integer REFERENCES hangzones,
  created_at timestamp NOT NULL,
  updated_at timestamp NOT NULL
);

CREATE TABLE friends (
  id serial PRIMARY KEY,
  user_hanger_id integer REFERENCES user_hangers NOT NULL,
  friend_user_hanger_id integer REFERENCES user_hangers NOT NULL,
  created_at timestamp NOT NULL,
  updated_at timestamp NOT NULL
);

CREATE TABLE request_hangs (
  id serial PRIMARY KEY,
  from_user_hanger_id integer REFERENCES user_hangers NOT NULL,
  to_user_hanger_id integer REFERENCES user_hangers NOT NULL,
  created_at timestamp NOT NULL,
  updated_at timestamp NOT NULL
);

