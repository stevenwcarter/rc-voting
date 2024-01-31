-- Your SQL goes here

CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE
);
CREATE UNIQUE INDEX uname ON users(username);

CREATE TABLE items (
    id INTEGER PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    done BOOL NOT NULL DEFAULT false
);

CREATE TABLE votes (
    user_id INTEGER NOT NULL NOT NULL,
    item_id INTEGER NOT NULL NOT NULL,
    ordinal INTEGER NOT NULL,

    PRIMARY KEY (user_id, item_id)
    FOREIGN KEY(user_id) REFERENCES users(id)
    FOREIGN KEY(item_id) REFERENCES items(id)
);
CREATE INDEX ballot ON votes(user_id ASC, ordinal ASC);
