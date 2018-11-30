import praw
import psycopg2
import os
from dotenv import load_dotenv
load_dotenv()

conn = psycopg2.connect("dbname='{0}' user='{1}' host='{2}' password='{3}'"
        .format(os.getenv("DB_NAME"), 
                os.getenv("DB_USER"), 
                os.getenv("DB_HOST"), 
                os.getenv("DB_PASSWORD")))

cur = conn.cursor()
cur.execute("DROP TABLE IF EXISTS person;")
cur.execute("DROP TABLE IF EXISTS person_reddit;")
cur.execute("CREATE TABLE person (id serial PRIMARY KEY, name varchar UNIQUE);")
cur.execute("CREATE TABLE person_reddit (id serial PRIMARY KEY, reddit_username varchar UNIQUE, person_id integer UNIQUE);")
cur.execute("INSERT INTO person (name) VALUES (%s)", ("abc",))
cur.execute("SELECT * FROM person;")
row = cur.fetchall()

conn.commit()
cur.close()
conn.close()


reddit = praw.Reddit(client_id=os.getenv("REDDIT_ID"),
        client_secret=os.getenv("REDDIT_SECRET"),
        user_agent=os.getenv("REDDIT_USER_AGENT"))

for comment in reddit.redditor('bob').comments.new(limit=10):
    print(comment.body)
    print(comment.created_utc)
