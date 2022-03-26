-- Add up migration script here
CREATE TYPE request_status AS ENUM ('awaiting_response', 'accepted', 'declined', 'cancelled');

CREATE TABLE request_hangs (
  id serial PRIMARY KEY,
  from_user_hanger_id integer REFERENCES user_hangers NOT NULL,
  to_user_hanger_id integer REFERENCES user_hangers NOT NULL,
  message text,
  hangzone_id integer REFERENCES hangzones NOT NULL,
  status request_status NOT NULL default 'awaiting_response',
  time timestamptz NOT NULL DEFAULT NOW(),
  created_at timestamptz NOT NULL DEFAULT NOW(),
  updated_at timestamptz NOT NULL DEFAULT NOW()
);

CREATE TABLE request_friends (
  id serial PRIMARY KEY,
  from_user_hanger_id integer REFERENCES user_hangers NOT NULL,
  to_user_hanger_id integer REFERENCES user_hangers NOT NULL,
  message text,
  status request_status NOT NULL default 'awaiting_response',
  created_at timestamptz NOT NULL DEFAULT NOW(),
  updated_at timestamptz NOT NULL DEFAULT NOW()
);

