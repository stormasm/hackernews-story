

Notes on files in the example directory...

1) top_n_items_redis writes the json data from the hackernews api to redis

2) redis_to_file writes the json data from redis to a file

3) story_ids writes just the ids to a file

4) story_title_delete removes the ids that do not have a title

5) story_tile creates a file with lines of json including the id and title


##### More details...

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
