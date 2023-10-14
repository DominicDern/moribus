use crate::utilities::stat::Stat;

pub struct Counter {
    id: u64,
    name: String,
    current_value: u32,
    max: u32,
}

impl Stat<(u32, u32)> for Counter {
    fn id(&self) -> &u64 {
        &self.id
    }

    fn value_str(&self) -> String {
        format!("{}/{}", &self.current_value, &self.max)
    }
}
