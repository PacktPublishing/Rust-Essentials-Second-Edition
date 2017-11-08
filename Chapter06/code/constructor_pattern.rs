struct Alien {
    name: &'static str,
    health: u32,
    damage: u32
}

impl Alien {
    fn new(s: &'static str, mut h: u32, d: u32) -> Self {
        if h > 100 { h = 100; }
        Alien { name: s, health: h, damage: d }
    }

    pub fn default() -> Self {
        Alien::new("Walker", 100, 10)
    }
    
    pub fn give_health(h: u32) -> Self {
        Alien::new("Zombie", h, 5)
    }
}   

fn main() {
    let al1 = Alien{ name: "Bork", health: 100, damage: 5 };
    let al2 = Alien::new("Berserk", 150, 15);
    println!("Alien 1 is a {} and inflicts {} damage points", al1.name, al1.damage);
    let al3 = Alien::default();
    println!("Alien 3 is a {} and inflicts {} damage points", al3.name, al3.damage);
    let al4 = Alien::give_health(75);
    println!("Alien 4 is a {} and inflicts {} damage points", al4.name, al4.damage);
}
// Alien 1 is a Bork and inflicts 5 damage points
// Alien 3 is a Walker and inflicts 10 damage points
// Alien 4 is a Zombie and inflicts 5 damage points

