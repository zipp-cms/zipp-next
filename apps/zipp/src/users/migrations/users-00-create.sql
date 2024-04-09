-- create users table
CREATE TABLE users (
    id text PRIMARY KEY,
    email text UNIQUE NOT NULL
);
