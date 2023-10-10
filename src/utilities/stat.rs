#[derive(Debug, PartialEq, Eq)]
pub enum StatValue {
    Value(i16),
    Modifier(i8),
    ModifierValue((i8, i16)),
    Counter(i16),
    // current value, default value
    Resource(i16, i16),
    // current value, default value
    Toggle(bool, bool),
}

#[derive(Debug)]
struct StatTimer {
    done: bool,
    turns_left: u16,
    target_turns: u16,
    stat_change: StatValue,
}

impl StatTimer {
    fn advance_turn(&mut self) -> &u16 {
        if self.turns_left == 1 {
            self.done = true;
        } else {
            self.turns_left -= 1;
        }
        &self.turns_left
    }

    fn get_turn(&self) -> &u16 {
        &self.turns_left
    }

    fn is_done(&self) -> &bool {
        &self.done
    }
}

#[derive(Debug)]
pub struct Stat {
    id: String,
    value: StatValue,
    timers: Vec<StatTimer>,
}

impl Stat {
    pub fn new(id: &str, value: StatValue) -> Stat {
        Stat {
            id: String::from(id),
            value,
            timers: Vec::new(),
        }
    }
}

impl Stat {
    pub fn change(&mut self, change_value: StatValue) -> &StatValue {
        match self.value {
            StatValue::Value(current_value) => match change_value {
                StatValue::Value(change_value) => {
                    self.value = StatValue::Value(current_value + change_value);
                }
                _ => {}
            },
            StatValue::Modifier(current_value) => match change_value {
                StatValue::Modifier(change_value) => {
                    self.value = StatValue::Modifier(current_value + change_value);
                }
                _ => {}
            },
            StatValue::ModifierValue(current_value) => match change_value {
                StatValue::ModifierValue(change_value) => {
                    self.value = StatValue::ModifierValue((
                        current_value.0 + change_value.0,
                        current_value.1 + change_value.1,
                    ))
                }
                _ => {}
            },
            StatValue::Counter(current_value) => match change_value {
                StatValue::Counter(_) => self.value = StatValue::Counter(current_value + 1),
                _ => {}
            },
            StatValue::Toggle(_, vanilla_value) => match change_value {
                StatValue::Toggle(new_value, _) => {
                    self.value = StatValue::Toggle(new_value, vanilla_value)
                }
                _ => {}
            },
            StatValue::Resource(current_value, max_value) => match change_value {
                StatValue::Resource(sub_value, _) => {
                    self.value = StatValue::Resource(current_value - sub_value, max_value);
                }
                _ => {}
            },
        }
        &self.value
    }
    pub fn revert(&mut self) -> &StatValue {
        match self.value {
            StatValue::Resource(_, max_value) => {
                self.value = StatValue::Resource(max_value, max_value);
            }
            StatValue::Toggle(_, vanilla_value) => {
                self.value = StatValue::Toggle(vanilla_value, vanilla_value);
            }
            _ => {}
        }
        &self.value
    }

    pub fn get_value(&self) -> &StatValue {
        &self.value
    }

    fn add_stat_timer(&mut self, stat_timer: StatTimer) -> &Vec<StatTimer> {
        &self.timers
    }
}

#[cfg(test)]
mod tests {
    use super::{Stat, StatValue};

    #[test]
    fn stat_change_value() {
        let current_value = StatValue::Value(10);
        let change_value = StatValue::Value(2);
        let comp_value = StatValue::Value(12);
        let mut test_stat = Stat::new("test_stat", current_value);
        let comp_stat = Stat::new("comp_stat", comp_value);
        test_stat.change(change_value);
        assert_eq!(test_stat.get_value(), comp_stat.get_value());
    }
    // TODO finish unit tests for all stat changes
}
