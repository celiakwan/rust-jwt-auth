CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR(50) NOT NULL,
  password VARCHAR(100) NOT NULL,
  role VARCHAR(50) NOT NULL,
  logged_in BOOLEAN NOT NULL DEFAULT FALSE,
  CONSTRAINT username_constraint UNIQUE (username)
);