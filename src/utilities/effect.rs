#[derive(Clone, Copy)]
pub enum EffectDuration {
    Turns(u8),
    LongRest,
    ShortRest,
    Perminent,
    Instantanious,
}

pub trait Effect {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn turns(&self) -> EffectDuration;
    fn turn_remainder(&self) -> EffectDuration;
}
