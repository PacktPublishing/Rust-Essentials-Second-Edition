use std::f32::consts;

static MAX_HEALTH: i32 = 100;
static GAME_NAME: &str = "Monster Attack";

fn main() {
	const MYPI: f32 = 3.14;
	println!("{}", MYPI);
    println!("{}", GAME_NAME);
        // use the PI value from the standard library:
	println!("{}", consts::PI);
}

// 3.14
// Monster Attack
// 3.141593