mod utilities;

use crate::utilities::roll::Roll;

fn main() {
    let ur_mom = Roll::roll("3d6, 1d12+8");
    print!("{}", ur_mom);
}
