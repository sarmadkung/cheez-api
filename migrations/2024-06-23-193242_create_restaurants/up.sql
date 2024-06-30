-- Your SQL goes here
CREATE TABLE IF NOT EXISTS restaurants (
  id UUID PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  location VARCHAR(255) NOT NULL,
  rating FLOAT8 NOT NULL
);
