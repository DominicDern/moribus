use super::stat::Stat;

/// Contains a value between 0 and {max}
pub struct Counter<T> {
    name: String,
    description: String,
    current_value: T,
    max: T,
}

impl Counter<u32> {
    fn current(&self) -> &u32 {
        &self.current_value
    }

    fn max(&self) -> &u32 {
        &self.max
    }

    fn add(&mut self, increment: u32) -> &u32 {
        if (self.current_value + increment) > self.max {
            self.current_value = self.max;
        } else {
            self.current_value = self.current_value + increment;
        }
        self.current()
    }

    fn subtract(&mut self, decrement: u32) -> &u32 {
        self.current_value = self.current_value - decrement;
        self.current()
    }

    fn max_add(&mut self, increment: u32) -> &u32 {
        self.max = self.max + increment;
        self.max()
    }

    fn is_zero(&self) -> bool {
        self.current_value == 0
    }
}

impl Stat<(u32, u32)> for Counter<u32> {
    fn name(&self) -> &String {
        let e = vec!(1, 2, 3);
        &self.name
    }
    fn value_str(&self) -> String {
        format!("{}/{}", &self.current(), &self.max())
    }
    fn description(&self) -> &String {
        &self.description
    }
}
