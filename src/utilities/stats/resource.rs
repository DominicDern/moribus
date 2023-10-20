use super::stat::Stat;

/// A quantity of a stat
pub struct Resource {
    name: String,
    description: String,
    quantity:u64,
}

impl Resource {}

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
