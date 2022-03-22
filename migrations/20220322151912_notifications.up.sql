-- Add up migration script here
CREATE TYPE notification_type AS ENUM ('hang', 'friend');

CREATE TABLE notifications (
  id serial PRIMARY KEY,
  user_hanger_id integer REFERENCES user_hangers NOT NULL,
  type notification_type NOT NULL,
  read boolean NOT NULL DEFAULT FALSE,
  trash boolean NOT NULL DEFAULT FALSE,
  created_at timestamptz NOT NULL DEFAULT NOW(),
  updated_at timestamptz NOT NULL DEFAULT NOW()
);
