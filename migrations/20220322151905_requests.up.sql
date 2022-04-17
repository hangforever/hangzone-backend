-- Add up migration script here
CREATE TYPE request_status AS ENUM ('awaiting_response', 'accepted', 'declined', 'cancelled');

CREATE TABLE request_hangs (
  id serial PRIMARY KEY,
  from_user_hanger_id integer REFERENCES user_hangers NOT NULL,
  to_user_hanger_id integer REFERENCES user_hangers NOT NULL,
  message text,
  hang_session_id integer REFERENCES hang_sessions NOT NULL,
  status request_status NOT NULL default 'awaiting_response',
  created_at timestamptz NOT NULL DEFAULT NOW()
);

CREATE TABLE request_friends (
  id serial PRIMARY KEY,
  from_user_hanger_id integer REFERENCES user_hangers NOT NULL,
  to_user_hanger_id integer REFERENCES user_hangers NOT NULL,
  message text,
  status request_status NOT NULL default 'awaiting_response',
  created_at timestamptz NOT NULL DEFAULT NOW()
);

