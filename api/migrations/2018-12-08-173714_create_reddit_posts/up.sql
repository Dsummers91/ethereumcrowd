CREATE TABLE reddit_posts (
  id UUID PRIMARY KEY,
  reddit_id UUID NOT NULL references reddit (id),
  post_id VARCHAR UNIQUE NOT NULL,
  body VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  score INTEGER NOT NULL DEFAULT 0,
  subreddit VARCHAR NOT NULL
)
