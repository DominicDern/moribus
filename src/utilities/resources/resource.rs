use super::stat::Stat;

/// A quantity of a stat
pub struct Resource {
    name: String,
    description: String,
    quantity: u64,
}

impl Resource {
    pub fn new(name: String, description: String) -> Resource {
        Resource { name, description, quantity: 0 }
    }
}

impl Resource {
    pub fn add(&mut self, number_added: u64) -> &u64 {
        self.quantity = self.quantity + number_added;
        &self.quantity
    }

    pub fn remove(&mut self, number_removed: u64) -> &u64 {
        self.quantity = self.quantity - number_removed;
        &self.quantity
    }
}

impl Stat<u64> for Resource {
    fn name(&self) -> &String {
        &self.name
    }

    fn value_str(&self) -> String {
        format!("{} | {}", self.quantity, self.name)
    }
    
    fn description(&self) -> &String {
        &self.description
    }
}
