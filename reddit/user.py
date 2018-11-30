import praw
import psycopg2
import os
from dotenv import load_dotenv
load_dotenv()

table_name=os.getenv("TABLE_NAME")

conn = psycopg2.connect("dbname='{0}' user='{1}' host='{2}' password='{3}'"
        .format(os.getenv("DB_NAME"), 
                os.getenv("DB_USER"), 
                os.getenv("DB_HOST"), 
                os.getenv("DB_PASSWORD")))


cur = conn.cursor()
#cur.execute("DROP TABLE IF EXISTS test;")
#cur.execute("CREATE TABLE " + table_name + " (id serial PRIMARY KEY, num integer UNIQUE, data varchar);")
#cur.execute("INSERT INTO " + table_name + " (num, data) VALUES (%s, %s)", (100, "abc'def"))
cur.execute("SELECT * FROM " + table_name + ";")
rows = cur.fetchall()

conn.commit()
cur.close()
conn.close()


reddit = praw.Reddit(client_id=os.getenv("REDDIT_ID"),
        client_secret=os.getenv("REDDIT_SECRET"),
        user_agent=os.getenv("REDDIT_USER_AGENT"))


for row in rows:
    print(row[1])
    for comment in reddit.redditor(row[1]).comments.new(limit=10):
        print(comment.body)
        print(comment.created_utc)
