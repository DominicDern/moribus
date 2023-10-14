use super::super::stat::Stat;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash)]
struct DescriptionProto<'a> {
    title: &'a str,
    value: &'a str,
}

impl<'a> DescriptionProto<'a> {
    fn new(title: &'a str, value: &'a str) -> DescriptionProto<'a> {
        DescriptionProto { title, value }
    }
}

#[derive(Debug)]
pub struct Description {
    id: u64,
    title: String,
    value: String,
}

impl Stat for Description {
    fn id(&self) -> &u64 {
        &self.id
    } 
    fn title(&self) -> &str {
        &self.title
    }
    fn value(&self) -> &str {
        &self.value
    }
}

impl Description {
    pub fn new(title: &str, value: &str) -> Description {
        let proto = DescriptionProto::new(title, value);
        let mut hasher = DefaultHasher::new();

        proto.hash(&mut hasher);

        Description { id: hasher.finish(), title: String::from(title), value: String::from(value) }
    }
}
