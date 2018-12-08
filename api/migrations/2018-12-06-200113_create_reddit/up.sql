CREATE TABLE reddit (
  id SERIAL PRIMARY KEY,
  person_id INTEGER UNIQUE NOT NULL references people (id),
  username VARCHAR UNIQUE NOT NULL
)
