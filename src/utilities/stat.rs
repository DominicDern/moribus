pub trait Stat {
    fn id(&self) -> &u64;
    fn title(&self) -> &str;
    fn value(&self) -> &str;
}
