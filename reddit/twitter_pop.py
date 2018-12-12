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

table_name="twitter"

api = twitter.Api(consumer_key=os.getenv("TWITTER_CONSUMER_KEY"),
        consumer_secret=os.getenv("TWITTER_CONSUMER_SECRET"),
        access_token_key=os.getenv("TWITTER_API_KEY"),
        access_token_secret=os.getenv("TWITTER_API_SECRET"))


users = api.GetUserTimeline(screen_name="ElonMusk")

print([u.text for u in users])

