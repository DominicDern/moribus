mod utilities;
mod spells;

use utilities::{spell_book::*, effect::EffectDuration};


// pub struct Spell {
//     name: String,
//     description: String,
//     school: String,
//     casting_time: CastingTime,
//     turns_elapsed: u8,
//     duration: EffectDuration,
//     /** verbal, somatic, materail */
//     components: (bool , bool, Option<Vec<Resource>>)

// }




fn main() -> Result<(), ()> {
    let thaumaturgy = Spell {
        name: "Thaumaturgy".to_string(),
        description: "You manifest a minor wonder, a sign of supernatural power, within range. You create one of the following magical effects within range:\n- Your voice booms up to three times as loud as normal for 1 minute.\n- You cause flames to flicker, brighten, dim, or change color for 1 minute.\n- You cause harmless tremors in the ground for 1 minute.\n- You create an instantaneous sound that originates from a point of your choice within range, such as a rumble of thunder, the cry of a raven, or ominous whispers.\n- You instantaneously cause an unlocked door or window to fly open or slam shut.\n- You alter the appearance of your eyes for 1 minute.\n- If you cast this spell multiple times, you can have up to three of its 1-minute effects active at a time, and you can dismiss such an effect as an action.".to_string(),
        school: "Transmutation".to_string(),
        casting_time: CastingTime::Turns(10),
        turns_elapsed: 0,
        duration: EffectDuration::Turns(10),
        components: (true, false, None),
    };

    Ok(())
}
