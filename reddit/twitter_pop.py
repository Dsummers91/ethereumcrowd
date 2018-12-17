import praw
import psycopg2
import requests
import os
import json
from pathlib import Path
from dotenv import load_dotenv
import twitter

## Load Env Variables from Root Dir
env_path = Path('../') / '.env'
load_dotenv(dotenv_path=env_path)
load_dotenv()

conn = psycopg2.connect("dbname='{0}' user='{1}' host='{2}' password='{3}'"
        .format(os.getenv("DB_NAME"), 
                os.getenv("DB_USER"), 
                os.getenv("DB_HOST"), 
                os.getenv("DB_PASSWORD")))


cur = conn.cursor()
cur.execute("SELECT username FROM twitter;")
rows = cur.fetchall()

conn.commit()
cur.close()
conn.close()

api = twitter.Api(consumer_key=os.getenv("TWITTER_CONSUMER_KEY"),
        consumer_secret=os.getenv("TWITTER_CONSUMER_SECRET"),
        access_token_key=os.getenv("TWITTER_API_KEY"),
        access_token_secret=os.getenv("TWITTER_API_SECRET"))


print("rows:")
for row in rows:
    timeline = api.GetUserTimeline(screen_name=row[0])
    print([u.text for u in timeline])


