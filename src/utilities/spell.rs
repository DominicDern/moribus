use crate::utilities::effect::Effect;

enum CastingTime {
    Action,
    BonusAction,
    Reaction,
    Minutes(i32),
}

pub struct Spell {
    name: String,
    casting_time: CastingTime,
    effects: Vec<Effect>,
}
