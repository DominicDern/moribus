pub struct Stat {
    name: String,
    value: u32,
    modifier: u32,
}

impl Stat {
    pub fn new(name: String, value: u32, modifier: u32) -> Stat {
        Stat { name, value, modifier }
    }
}

impl Stat {
    pub fn get(&self) -> (&str, &u32, &u32) {
        (&self.name, &self.value, &self.modifier)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &u32 {
        &self.value
    }

    pub fn modifier(&self) -> &u32 {
        &self.modifier
    }
}
