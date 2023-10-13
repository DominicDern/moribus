use super::super::stat::Stat;

pub struct Description {
    id: u64,
    value: String,
}

impl Stat for Description {
    fn id(&self) -> &u64 {
        &self.id
    } 
    fn value(&self) -> &str {
        &self.value
    }
}

impl Description {
    pub fn new(value: String) -> Description {
        let mut id: u64 = 0;
        let mut total_val: u64 = 0;
        for char in value.as_bytes() {
            total_val += (char * 1) as u64;
        }
        wh
        for char in value.as_bytes() {
            id += (char * 27) as u64;
        }
        Description { id, value }
    }
}
