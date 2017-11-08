struct Alien {
    name: &'static str,
    health: u32,
    damage: u32
}

struct AlienBuilder {
    name: &'static str,
    health: u32,
    damage: u32
}

impl AlienBuilder {
    fn new() -> Self {
        AlienBuilder { name: "Walker", health: 100, damage: 10 }
    }

    fn name(&mut self, n: &'static str) -> &mut AlienBuilder {
        self.name = n;
        self
    }

    fn health(&mut self, h: u32) -> &mut AlienBuilder {
        self.health = h;
        self
    }

    fn damage(&mut self, d: u32) -> &mut AlienBuilder {
        self.damage = d;
        self
    }

    fn finish(&self) -> Alien {
        Alien { name: self.name, health: self.health, damage: self.damage }
    }
}

fn main() {
    let al1 = AlienBuilder::new()
                .name("Bork")
                .health(80)
                .damage(20)
                .finish();
    println!("name: {}", al1.name);
    println!("health: {}", al1.health);
}
// name: Bork
// health: 80