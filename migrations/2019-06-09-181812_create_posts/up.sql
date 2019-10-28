-- Your SQL goes here
CREATE TABLE posts (
  id INTEGER PRIMARY KEY NOT NULL,
  author INTEGER NOT NULL,
  published_content INTEGER
);

CREATE TABLE post_contents (
  id INTEGER PRIMARY KEY NOT NULL,
  post_id INTEGER NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL
);