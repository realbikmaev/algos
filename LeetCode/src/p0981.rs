// 981. Time Based Key-Value Store
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
struct TimeMap {
    tm: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap { tm: HashMap::new() }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.tm
            .entry(key)
            .or_insert(vec![])
            .push((timestamp, value));
    }

    fn get(&mut self, key: String, timestamp: i32) -> String {
        if let Some(s) = self.tm.get(&key) {
            return match s.binary_search_by_key(&timestamp, |&(ts, _)| ts) {
                Ok(i) => s[i].1.clone(),
                Err(i) => {
                    if i == 0 {
                        String::from("")
                    } else {
                        s[i - 1].1.clone()
                    }
                }
            };
        }
        return String::from("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_time_map_foobar() {
        let mut t = TimeMap::new();
        t.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(t.get("foo".to_string(), 1), "bar".to_string());
        assert_eq!(t.get("foo".to_string(), 3), "bar".to_string());
        t.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(t.get("foo".to_string(), 4), "bar2".to_string());
        assert_eq!(t.get("foo".to_string(), 5), "bar2".to_string());
    }

    #[test]
    fn check_time_map_love() {
        let mut t = TimeMap::new();
        t.set("love".to_string(), "high".to_string(), 10);
        t.set("love".to_string(), "low".to_string(), 20);
        assert_eq!(t.get("love".to_string(), 5), "".to_string());
        assert_eq!(t.get("love".to_string(), 10), "high".to_string());
        assert_eq!(t.get("love".to_string(), 15), "high".to_string());
        assert_eq!(t.get("love".to_string(), 20), "low".to_string());
        assert_eq!(t.get("love".to_string(), 25), "low".to_string());
    }
}
