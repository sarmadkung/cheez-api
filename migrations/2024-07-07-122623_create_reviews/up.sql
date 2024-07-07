-- Your SQL goes here
CREATE TABLE IF NOT EXISTS reviews (
  id UUID PRIMARY KEY,
  body TEXT NOT NULL,
  rating FLOAT8 NOT NULL,
  item_id UUID NULL REFERENCES items(id),
  user_id UUID NULL REFERENCES users(id),
  restaurant_id UUID NULL REFERENCES restaurants(id)
);
