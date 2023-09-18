struct IndividualRoll {
    number_of_rolls: Option<u8>,
    die_size: u8,
    modifier: Option<u16>,
}

impl IndividualRoll {
    fn new(number_of_rolls: Option<u8>, die_size: u8, modifier: Option<u16>) -> IndividualRoll {
        IndividualRoll {
            number_of_rolls,
            die_size,
            modifier,
        }
    }
}

pub struct Roll {
    roll: Vec<IndividualRoll>,
}

impl Roll {
    fn parse_roll(roll: &str) -> Roll {
        let roll = "d10 + 5, 3d6 + 2";
        let mut roll_vec: Vec<IndividualRoll> = Vec::new();

        let roll: Vec<&str> = roll.split(',').collect();
        for roll in roll.clone() {
            let number_of_rolls: Option<u8>;
            let die_size: u8;
            let modifier: Option<u16>;

            let mut roll: Vec<&str> = roll.split('+').collect();
            if roll.len() == 2 {
                modifier = Some(roll[1].trim().parse().unwrap());
            } else {
                modifier = None;
            }
            if roll.len() > 1 {
                roll.pop();
            }
            let roll = String::from(roll[0]);

            let roll: Vec<&str> = roll.split('d').collect();
            if roll[0] == "" {
                number_of_rolls = None;
                die_size = roll[1].trim().parse().unwrap();
            } else {
                number_of_rolls = Some(roll[0].trim().parse().unwrap());
                die_size = roll[1].trim().parse().unwrap();
            }
            let roll = IndividualRoll::new(number_of_rolls, die_size, modifier);
            roll_vec.push(roll);
        }
        Roll::new(roll_vec)
    }

    fn new(roll_vec: Vec<IndividualRoll>) -> Roll {
        Roll { roll: roll_vec }
    }
}
