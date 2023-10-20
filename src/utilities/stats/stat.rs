pub trait Stat<T> {
    fn name(&self) -> &String;
    fn description(&self) -> &String;
    fn value_str(&self) -> String;
}
