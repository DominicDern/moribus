use super::stat::Stat;

pub struct Toggle {
    name: String,
    description: String,
    value: bool,
    override_value: bool,
}

impl Toggle {
    pub fn new(name: &str, description: &str, value: bool) -> Toggle {
        Toggle { name: String::from(name), description: description.to_string(), value, override_value: value }
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
    fn name(&self) -> &String {
        &self.name
    }

    fn value_str(&self) -> String {
        format!("{}", &self.value)
    }

    fn description(&self) -> &String {
        &self.description
    }
}
