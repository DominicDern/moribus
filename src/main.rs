use crate::utilities::stat::Stat;

mod utilities;

fn main() -> Result<(), ()> {
    use utilities::stat_types::toggle::Toggle;
    let description = Toggle::new("hi", false);
    print!("{}/{:#?}", description.value_str(), description.override_value);
    Ok(())
}
