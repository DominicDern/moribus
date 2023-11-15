use super::{effect::Effect, resources::stat, stat::Stat};

pub struct Character {
    /// current movement, max movevent, temp movement
    movement: (f32, f32, Option<f32>),
    stats: Vec<Stat>,
    // effects: Box<>
}
