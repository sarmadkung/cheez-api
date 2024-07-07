-- Your SQL goes here
CREATE TABLE IF NOT EXISTS menu (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id),
    restaurant_id UUID NOT NULL REFERENCES restaurants(id)
)
