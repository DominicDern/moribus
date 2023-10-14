struct ActionInventory {
    movement: (f32, u16),
}

impl ActionInventory {
    fn set_movement(&mut self, used_movement: f32) -> &(f32, u16) {
        if self.movement.0 - used_movement < 0. {
            self.movement.0 = 0.;
        }
        &self.movement
    }

    fn get_movement(&self) -> &(f32, u16) {
        &self.movement
    }
}

pub struct Character {}
