-- Add up migration script here
CREATE TABLE hang_sessions (
  id serial PRIMARY KEY,
  name text NOT NULL,
  description text DEFAULT '',
  hangzone_id integer REFERENCES hangzones NOT NULL,
  starts_at timestamptz NOT NULL DEFAULT NOW()
);

CREATE TABLE hangers (
  id serial PRIMARY KEY,
  hang_session_id integer REFERENCES hang_sessions NOT NULL,
  user_hanger_id integer REFERENCES user_hangers NOT NULL,
  host boolean DEFAULT false,
  joined_at timestamptz NOT NULL DEFAULT NOW()
);

ALTER TABLE request_hangs
ADD COLUMN hang_session_id integer REFERENCES hang_sessions NOT NULL;
