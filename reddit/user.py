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
cur.execute("DROP TABLE IF EXISTS test;")
cur.execute("CREATE TABLE test (id serial PRIMARY KEY, num integer UNIQUE, data varchar);")
cur.execute("INSERT INTO test (num, data) VALUES (%s, %s)", (100, "abc'def"))
cur.execute("SELECT * FROM test;")
row = cur.fetchall()
print(row)
conn.commit()
cur.close()
conn.close()


reddit = praw.Reddit(client_id=os.getenv("REDDIT_ID"),
        client_secret=os.getenv("REDDIT_SECRET"),
        user_agent=os.getenv("REDDIT_USER_AGENT"),
        username=os.getenv("REDDIT_USERNAME"),
        password=os.getenv("REDDIT_PASSWORD"))

for comment in reddit.redditor('bob').comments.new(limit=10):
    print(comment.body)
    print(comment.created_utc)
