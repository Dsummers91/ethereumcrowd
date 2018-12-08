CREATE TABLE reddit_posts (
  id SERIAL PRIMARY KEY,
  reddit_id INTEGER UNIQUE NOT NULL references reddit (id),
  post_id INTEGER UNIQUE NOT NULL,
  body VARCHAR NOT NULL,
  create_time TIMESTAMP NOT NULL
)
