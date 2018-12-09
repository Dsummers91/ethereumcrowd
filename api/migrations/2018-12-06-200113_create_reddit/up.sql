CREATE TABLE reddit (
  id UUID PRIMARY KEY,
  person_id UUID UNIQUE NOT NULL references people (id),
  username VARCHAR UNIQUE NOT NULL
)
