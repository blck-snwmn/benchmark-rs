use std::collections::HashMap;

pub fn do_something() {
    println!("something")
}

pub trait Matcher {
    fn do_match(&self, i: i32) -> Option<i32>;
}

pub struct MatchMatcher {}

impl MatchMatcher {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MatchMatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Matcher for MatchMatcher {
    fn do_match(&self, i: i32) -> Option<i32> {
        match i {
            1 => Some(i * 2),
            3 => Some(i * 2),
            5 => Some(i * 2),
            7 => Some(i * 2),
            9 => Some(i * 2),
            11 => Some(i * 2),
            _ => None,
        }
    }
}
pub struct HashmapMatcher {
    map: HashMap<i32, i32>,
}

impl HashmapMatcher {
    pub fn new() -> Self {
        let map = vec![(1, 2), (3, 6), (5, 10), (7, 14), (9, 18), (11, 22)]
            .into_iter()
            .collect();
        Self { map }
    }
}

impl Default for HashmapMatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Matcher for HashmapMatcher {
    fn do_match(&self, i: i32) -> Option<i32> {
        self.map.get(&i).copied()
    }
}

pub struct FnvHashmapMatcher {
    map: fnv::FnvHashMap<i32, i32>,
}

impl FnvHashmapMatcher {
    pub fn new() -> Self {
        let map = vec![(1, 2), (3, 6), (5, 10), (7, 14), (9, 18), (11, 22)]
            .into_iter()
            .collect();
        Self { map }
    }
}

impl Default for FnvHashmapMatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Matcher for FnvHashmapMatcher {
    fn do_match(&self, i: i32) -> Option<i32> {
        self.map.get(&i).copied()
    }
}
