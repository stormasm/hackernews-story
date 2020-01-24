use r2d2_redis::{r2d2, RedisConnectionManager};
use redis::{Commands, RedisResult};

use std::fs::File;
use std::io::{Error, Write};

fn get_hashmap_keys(key: String) -> RedisResult<Vec<u32>> {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let pool = pool.clone();
    let mut con = pool.get().unwrap();
    con.hkeys(key)
}

fn main() -> Result<(), Error> {
    let hn_story_key = "hn-story-20";

    let mut keys = get_hashmap_keys(hn_story_key.to_string()).unwrap();
    keys.sort();

    let path = "/tmp13/hackernews-story-archive/data/story-ids.json";
    let mut output = File::create(path)?;

    write!(output, "{}", "[")?;

    for key in &keys {
        write!(output, "{}", key.to_string())?;
        write!(output, "{}", ",")?;
    }

    write!(output, "{}", "]")?;

    output.sync_all()?;
    println!("Number of keys = {}", keys.len());
    Ok(())
}
