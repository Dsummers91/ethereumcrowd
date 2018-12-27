import praw
import psycopg2
import requests
import os
import json
from pathlib import Path
from dotenv import load_dotenv

## Load Env Variables from Root Dir
env_path = Path('../') / '.env'
load_dotenv(dotenv_path=env_path)
load_dotenv()

table_name=os.getenv("TABLE_NAME")

conn = psycopg2.connect("dbname='{0}' user='{1}' host='{2}' password='{3}'"
    .format(os.getenv("DB_NAME"), 
      os.getenv("DB_USER"), 
      os.getenv("DB_HOST"), 
      os.getenv("DB_PASSWORD")))


cur = conn.cursor()
cur.execute("SELECT * FROM " + table_name + ";")
rows = cur.fetchall()

conn.commit()
cur.close()
conn.close()

print(rows)

reddit = praw.Reddit(client_id=os.getenv("REDDIT_ID"),
    client_secret=os.getenv("REDDIT_SECRET"),
    user_agent=os.getenv("REDDIT_USER_AGENT"))


for row in rows:
  for comment in reddit.redditor(row[2]).comments.new(limit=10):
    r = requests.post("http://localhost:8000/reddit/comments", json={
      'username': row[2], 
      'comment_id': comment.id, 
      'score': comment.score, 
      'submission_title': comment.submission.title,
      'subreddit': comment.subreddit.display_name,
      'body': comment.body,
      }, headers={'Content-type': 'application/json'})
    pass 

  for submission in reddit.redditor(row[2]).submissions.new(limit=10):
    r = requests.post("http://localhost:8000/reddit/posts", json={
      'username': row[2], 
      'post_id': submission.id, 
      'title': submission.title,
      'body': submission.selftext,
      'score': submission.score,
      'subreddit': submission.subreddit.display_name
      }, headers={'Content-type': 'application/json'})
