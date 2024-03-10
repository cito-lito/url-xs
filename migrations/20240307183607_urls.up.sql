-- Add up migration script here

CREATE TABLE urls (
  id CHAR(8) PRIMARY KEY,
  long_url VARCHAR(2048) NOT NULL,
  user_id CHAR(8) NOT NULL,
  created_at TIMESTAMP NOT NULL
);
