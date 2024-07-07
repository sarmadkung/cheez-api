-- Your SQL goes here
CREATE TABLE IF NOT EXISTS menu (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    restaurant_id UUID NOT NULL REFERENCES restaurants(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);