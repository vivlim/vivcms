-- Your SQL goes here
CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    pass_sha VARCHAR NOT NULL,
    salt VARCHAR NOT NULL
)