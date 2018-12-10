CREATE TABLE reddit_posts (
  id UUID PRIMARY KEY,
  reddit_id UUID UNIQUE NOT NULL references reddit (id),
  post_id VARCHAR UNIQUE NOT NULL,
  body VARCHAR NOT NULL
)
