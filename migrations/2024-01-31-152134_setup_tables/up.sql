-- Your SQL goes here

CREATE TABLE users (
    uuid TEXT PRIMARY KEY NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);
CREATE UNIQUE INDEX email_unique ON users(email);

CREATE TABLE items (
    uuid TEXT NOT NULL PRIMARY KEY,
    election_uuid TEXT NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    done BOOL NOT NULL DEFAULT false,
    FOREIGN KEY(election_uuid) REFERENCES elections(uuid)
);
CREATE INDEX election_id_items_idx on items(election_uuid);

CREATE TABLE votes (
    election_uuid TEXT NOT NULL,
    user_uuid TEXT NOT NULL,
    item_uuid TEXT NOT NULL,
    ordinal INTEGER NOT NULL,

    PRIMARY KEY (election_uuid, user_uuid, item_uuid)
    FOREIGN KEY(election_uuid) REFERENCES elections(uuid)
    FOREIGN KEY(user_uuid) REFERENCES users(uuid)
    FOREIGN KEY(item_uuid) REFERENCES items(uuid)
);
CREATE INDEX ballot ON votes(user_uuid ASC, ordinal ASC);

CREATE TABLE elections (
  uuid TEXT PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  owner_uuid TEXT NOT NULL,
  FOREIGN KEY(owner_uuid) REFERENCES users(uuid)
);

CREATE TABLE sessions (
  uuid TEXT NOT NULL PRIMARY KEY,
  user_uuid TEXT NOT NULL,
  created BIGINT NOT NULL,
  expires BIGINT NOT NULL,
  data TEXT,
  FOREIGN KEY(user_uuid) REFERENCES users(uuid)
) WITHOUT ROWID;
