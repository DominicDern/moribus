use super::{effect::{Effect, EffectDuration}, resources::resource::Resource};

#[derive(Clone, Copy)]
pub enum CastingTime {
    Action,
    BonusAction,
    Reaction,
    Turns(i32),
}

pub struct Spell {
    pub name: String,
    pub description: String,
    pub school: String,
    pub casting_time: CastingTime,
    pub turns_elapsed: u8,
    pub duration: EffectDuration,
    /** verbal, somatic, materail */
    pub components: (bool , bool, Option<Vec<Resource>>) 
}

impl Effect for Spell {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn turns(&self) -> EffectDuration {
        self.duration.clone()
    }

    fn turn_remainder(&self) -> EffectDuration {
        match self.duration {
            EffectDuration::Turns(turns) => {
                let turns_left = turns.clone() - &self.turns_elapsed.clone();
                EffectDuration::Turns(turns_left)
            },
            _ => self.duration.clone()
        }
    }
}

impl Spell {
    pub fn casting_time(&self) -> CastingTime {
        self.casting_time.clone()
    }
    pub fn components(&self) -> &(bool, bool, Option<Vec<Resource>>) {
        &self.components
    }
}
