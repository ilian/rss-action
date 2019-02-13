use std::collections::HashMap;
use std::io::{self, BufRead, Write, Seek};
use std::fs::{self, File};
use std::error::Error;

pub struct LastSeen {
    map: HashMap<String, String>,
    file: File
}

impl LastSeen {
    pub fn new(path: &str) -> Result<LastSeen, Box<Error>> {
        let file = fs::OpenOptions::new().read(true)
                                         .write(true)
                                         .create(true)
                                         .open(path)?;
        let mut map: HashMap<String, String> = HashMap::new();
        for line_iter in io::BufReader::new(&file).lines() {
            let line = line_iter?;
            if line.is_empty() {
                continue;
            }
            let mut kv = line.split_whitespace();
            let key = kv.next().ok_or("No key found")?;
            let value = kv.next().ok_or("No value found")?;
            // TODO: Check if kv.next() is None
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(LastSeen {
            map,
            file
        })
    }

    fn get_last_seen(&self, feed: &str) -> Option<&str> {
        self.map.get(feed).map(|x| x.as_str())
    }

    pub fn set_last_seen(&mut self, feed: &str, item_url: &str) -> Result<(), Box<Error>> {
        self.map.insert(feed.to_owned(), item_url.to_owned());
        self.file.set_len(0)?;
        self.file.seek(io::SeekFrom::Start(0))?;
        for (key, val) in self.map.iter() {
            writeln!(self.file, "{} {}", key, val)?;
        }
        Ok(())
    }

    pub fn get_unvisited<'a,'b>(&self, feed: &str, item_urls: &'a[&'b str]) -> &'a[&'b str] {
        let mut idx = 0;
        if let Some(last_seen_url) = self.get_last_seen(feed) {
            if let Some(last_idx) = item_urls.iter().position(|x| x == &last_seen_url) {
                idx = last_idx;
            }
        }
        &item_urls[idx+1..]
    }
}
