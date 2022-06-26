-- Add down migration script here
ALTER TABLE request_hangs
DELETE COLUMN hang_session_id integer REFERENCES hang_sessions NOT NULL;

DROP TABLE hangers;
DROP TABLE hang_sessions;
