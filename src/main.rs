mod utilities;

use utilities::stats::toggle;

use crate::utilities::stats::stat::Stat;

fn main() -> Result<(), ()> {
    let ur_mom = toggle::Toggle::new("ur_mom", "ur_dad", true);
    print!("{}", ur_mom.value_str());
    Ok(())
}
