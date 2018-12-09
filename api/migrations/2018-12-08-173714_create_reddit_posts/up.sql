CREATE TABLE reddit_posts (
  id UUID PRIMARY KEY,
  reddit_id UUID UNIQUE NOT NULL references reddit (id),
  post_id UUID UNIQUE NOT NULL,
  body VARCHAR NOT NULL,
  create_time TIMESTAMP NOT NULL
)
