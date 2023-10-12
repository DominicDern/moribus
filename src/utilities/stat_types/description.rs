use super::super::stat::Stat;

pub struct Description {
    id: String,
    value: String,
}

impl Stat for Description {
    fn id(&self) -> &str {
        &self.id
    } 
    fn value(&self) -> &str {
        &self.value
    }
}
