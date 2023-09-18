mod utilities;

use utilities::roll;

fn main() {
    let e = roll::Roll::roll("1d10 + 2 , d6");
    print!("{e}");
}
