-- Your SQL goes here
CREATE TABLE posts (
  id INTEGER PRIMARY KEY NOT NULL,
  author INTEGER NOT NULL
);

CREATE TABLE post_contents (
  post_id INTEGER NOT NULL,
  revision INTEGER NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  PRIMARY KEY (post_id, revision)
);