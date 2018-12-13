CREATE TABLE people (
  id uuid NOT NULL PRIMARY KEY,
  name VARCHAR UNIQUE NOT NULL
);

CREATE UNIQUE INDEX name_unique_idx on people (LOWER(name));
