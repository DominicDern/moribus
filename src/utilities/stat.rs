pub trait Stat<T> {
    fn id(&self) -> &u64;
    fn value_str(&self) -> String;
}
