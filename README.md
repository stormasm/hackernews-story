
##### redis

The redis data are dump.rdb files created via
[top_n_items_redis](https://github.com/stormasm/hackernews-story-archive/blob/master/examples/top_n_items_redis.rs)

We need to reprocess everything next time and remove these cases
along with making sure that the stories we store in Redis has a **title**

For now I will attempt to hand remove from Redis the following ids
that look like this....

https://hacker-news.firebaseio.com/v0/item/21948540.json?print=pretty  
https://hacker-news.firebaseio.com/v0/item/21949067.json?print=pretty  
https://hacker-news.firebaseio.com/v0/item/21949136.json?print=pretty  
https://hacker-news.firebaseio.com/v0/item/21949339.json?print=pretty  

Next time through on the processing these IDs should not be in there.
