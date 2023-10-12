pub trait Stat {
    fn id(&self) -> &str;
    fn value(&self) -> &str;
}
