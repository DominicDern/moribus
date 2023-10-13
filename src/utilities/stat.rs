pub trait Stat {
    fn id(&self) -> &u64;
    fn value(&self) -> &str;
}
