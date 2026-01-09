use bytes::Bytes;
use std::collections::HashMap;

pub struct Db {
    entries: HashMap<String, Bytes>,
}

impl Db {
    pub fn new() -> Db {
        Db {
            entries: HashMap::new(),
        }
    }

    pub fn write(&mut self, arr: &[String]) -> Result<&str, &'static str> {
        let key = &arr[1];
        let value = &arr[2];

        let val = value.clone();
        let res: &Option<Bytes> = &self.entries.insert(String::from(key), Bytes::from(val));

        match res {
            Some(_res) => Ok("r Ok"),
            None => Ok("Ok"),
        }
    }

    pub fn read(&mut self, arr: &[String]) -> Result<&str, &'static str> {
        let key = &arr[1];
        let query_result = self.entries.get(key);

        if let Some(value) = query_result {
            match str::from_utf8(value) {
                Ok(v) => Ok(v),
                Err(_) => Err("no such key found"),
            }
        } else {
            return Err("no such key found");
        }
    }

    pub fn delete(&mut self, arr: &[String]) -> Result<&str, &'static str> {
        let key = &arr[1];
        let res = self.entries.remove(key);

        match res {
            Some(_res) => Ok("r Ok"),
            None => Ok("Ok"),
        }
    }
}
