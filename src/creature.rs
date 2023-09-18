enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

struct Creature {
    name: String,
    size: Size,
    alignment: Option<String>,
    armor_class: u8,
    hit_points: u32,
}
