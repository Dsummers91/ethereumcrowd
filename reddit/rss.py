import feedparser

d = feedparser.parse('https://www.coindesk.com/feed')

print(d['feed']['title'])

for entry in d['entries']:
  print(entry['title'])
  print(entry['description'])


