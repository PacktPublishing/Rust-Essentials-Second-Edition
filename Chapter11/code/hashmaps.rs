use std::collections::HashMap;

fn main() { 
    let mut monsters = HashMap::new();

    monsters.insert("Oron", "Uranus");
    monsters.insert("Cyclops", "Venus");
    monsters.insert("Rahav", "Neptune");
    monsters.insert("Homo Sapiens", "Earth");

    match monsters.get(&"Rahav") {
        Some(&planet) => println!("Rahav originates from: {}", planet),
        _ => println!("Planet of Rahav unknown."),
    }

    monsters.remove(&("Homo Sapiens")); 

    // `HashMap::iter()` returns 
    for (monster, planet) in monsters.iter() {
        println!("Monster {} originates from planet {}", monster, planet); 
    }
}
// Rahav originates from: Neptune
// Monster Rahav originates from planet Neptune
// Monster Cyclops originates from planet Venus
// Monster Oron originates from planet Uranus