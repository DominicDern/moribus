use crate::utilities::stat::Stat;

mod utilities;

fn main() -> Result<(), ()> {
    use utilities::stat_types::description::Description;
    let description = Description::new("hi", "mom");
    print!("{:#?}", description.value());
    Ok(())
}
