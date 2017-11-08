// code from Chapter 11/code/paths.rs:
use std::path::Path;

fn main() {
    let path = Path::new("hello.txt");
 let display = path.display();
    
    // test whether path exists: 
    if path.exists() {
        println!("{} exists", display);
    }
    else {
        panic!("This path or file does not exist!");
    }

    let file = path.file_name().unwrap();
    let extension = path.extension().unwrap();
    let parent_dir = path.parent().unwrap();
    println!("This is file {:?} with extension {:?} in folder {:?}", file, extension, parent_dir);

    // Check if the path is a file
    if path.is_file() { println!("{} is a file", display);  }
    // Check if the path is a directory
    if path.is_dir() { println!("{} is a directory", display); }

    // `join` merges a path with a byte container using the OS specific
    // separator, and returns the new path
    let new_path = path.join("abc").join("def");

// Convert the path into a string slice
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }

}
