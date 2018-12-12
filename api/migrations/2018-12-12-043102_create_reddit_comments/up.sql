CREATE TABLE reddit_comments (
  id UUID PRIMARY KEY,
  reddit_id UUID NOT NULL references reddit (id),
  comment_id VARCHAR UNIQUE NOT NULL,
  body VARCHAR NOT NULL,
  score INTEGER NOT NULL DEFAULT 0,
  subreddit VARCHAR NOT NULL
)
