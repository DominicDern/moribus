use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::super::stat::Stat;

#[derive(Hash)]
struct ToggleProto<'a> {
    name: &'a str,
    value: &'a bool,
}

impl<'a> ToggleProto<'a> {
    fn new(name: &'a str, value: &'a bool) -> ToggleProto<'a> {
        ToggleProto { name, value }
    }
}

pub struct Toggle {
    id: u64,
    name: String,
    value: bool,
    override_value: bool,
}

impl Toggle {
    pub fn new(name: &str, value: bool) -> Toggle {
        let proto = ToggleProto::new(name, &value);
        let mut hasher = DefaultHasher::new();

        proto.hash(&mut hasher);

        Toggle { id: hasher.finish(), name: String::from(name), value, override_value: value }
    }
}

impl Toggle {
    pub fn toggle(&mut self) -> &bool {
        self.value = !self.value;
        &self.value
    }
    
    pub fn set_override(&mut self, new_value: bool) -> &bool {
        self.value = new_value;
        &self.value
    }

    pub fn get_override(&self) -> &bool {
        &self.override_value
    }
}

impl Stat<bool> for Toggle {
    fn id(&self) -> &u64 {
        &self.id
    }

    fn value_str(&self) -> String {
        format!("{}", &self.value)
    }
}
