-- Your SQL goes here
CREATE TABLE items IF NOT EXISTS {
  id UUID PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  price FLOAT8 NOT NULL,
  about TEXT NULL,
  time_to_make INT NULL,
  rating FLOAT8 NULL DEFAULT 0,
  menu_id UUID NOT NULL REFERENCES menus(id)
  restaurant_id UUID NOT NULL REFERENCES restaurants(id)
};
