use super::stats;

pub struct SavingThrow {}

pub enum EffectDuration {
    Perminent,
    ShortRest,
    LongRest,
    Turns(u8),
}

pub struct Effect {
    id: String,
    stat_change_id: Option<Box<dyn stats::stat::Stat<i32>>>,
    duration: EffectDuration,
    saving_throws: Option<SavingThrow>,
    description: String,
    added_action_id: Option<String>,
}

impl Effect {
    pub fn new(
        id: String,
        stat_change_id: Option<Box<dyn stats::stat::Stat<i32>>>,
        duration: EffectDuration,
        saving_throws: Option<SavingThrow>,
        description: String,
        added_action_id: Option<String>,
    ) -> Effect {
        Effect {
            id,
            stat_change_id,
            duration,
            saving_throws,
            description,
            added_action_id,
        }
    }
}

impl Effect {
    fn apply() {}
}
