use std::collections::HashSet;

fn main() {
    let mut m1: HashSet<&str> = vec!["Cyclops", "Raven", "Gilgamesh"].into_iter().collect();
    let m2: HashSet<&str> = vec!["Moron", "Keshiu", "Raven"].into_iter().collect();

    m1.insert("Moron");
    if m1.insert("Raven") {
        println!("New value added")
    }
    else {
        println!("This value is already present")
    }
    
    println!("m1: {:?}", m1);

    println!("Intersection: {:?}", m1.intersection(&m2).collect::<Vec<_>>());
    println!("Union: {:?}", m1.union(&m2).collect::<Vec<_>>());
    println!("Difference: {:?}", m1.difference(&m2).collect::<Vec<_>>());
    println!("Symmetric Difference: {:?}",
             m1.symmetric_difference(&m2).collect::<Vec<_>>());
}
// This value is already present
// m1: {"Moron", "Raven", "Cyclops", "Gilgamesh"}
// Intersection: ["Moron", "Raven"]
// Union: ["Moron", "Gilgamesh", "Raven", "Cyclops", "Keshiu"]
// Difference: ["Gilgamesh", "Cyclops"]
// Symmetric Difference: ["Cyclops", "Gilgamesh", "Keshiu"]
