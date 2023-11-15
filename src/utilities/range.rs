pub enum Range {
    /// Either only effecting the user or a radius centered on the user and what other types of
    /// entities it can effect
    User(Option<(u16, String)>),
    Touch,
    Feet(u16)
}
