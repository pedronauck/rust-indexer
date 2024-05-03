-- Add up migration script here

DROP TABLE IF EXISTS blocks;
CREATE TABLE blocks (
  id INTEGER PRIMARY KEY,
  data BYTEA NOT NULL UNIQUE
);

CREATE INDEX blocks_id_idx ON blocks (id);
