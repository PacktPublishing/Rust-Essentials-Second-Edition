// this program DOES NOT COMPILE: on purpose
struct Alien {
    name: String,
    no_tentacles: u8
}

struct Tentacle {
    poison: u8,
    owner: Alien
}

fn main() {
    let dhark = Alien { name: "Dharkalen".to_string(), no_tentacles: 7 };

    // defining dhark's tentacles:
    for i in 0u8..dhark.no_tentacles {
    	// error in following line:
    	// use of moved value 'dhark'
    	// note: move occurs because `dhark` has type `Alien`, which does not implement the `Copy` trait
//        Tentacle { poison: i * 3, owner: dhark };
    }
}