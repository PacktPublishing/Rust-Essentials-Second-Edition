use std::env;
use std::fs;
use std::error::Error;

fn main() {
    show_dir().unwrap();
}

fn show_dir() -> Result<(), Box<Error>> {
    let here = try!(env::current_dir());
    println!("Contents in: {}", here.display());
    for entry in try!(fs::read_dir(&here)) {
        let path = try!(entry).path();
        let md = try!(fs::metadata(&path));
        println!("  {} ({} bytes)", path.display(), md.len());
    }
    Ok(())
}
// Contents in: F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\a (0 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\filesystem.rs (3385 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\hello.txt (22 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\info.txt (15 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\lorem_ipsum.txt (894 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\lorem_ipsum2.txt (447 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\numbers.txt (22 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\paths.exe (141824 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\paths.pdb (1183744 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\paths.rs (1155 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\reading_text_file.exe (239104 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\reading_text_file.pdb (1273856 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\reading_text_file.rs (796 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\read_file.exe (146944 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\read_file.pdb (1191936 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\read_file.rs (710 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\read_files_in_dir.exe (151552 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\read_files_in_dir.pdb (1200128 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\read_files_in_dir.rs (1045 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\read_file_try.exe (146944 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\read_file_try.pdb (1183744 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\read_file_try.rs (628 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\write_file.exe (137728 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\write_file.pdb (1175552 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\write_file.rs (1815 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\write_file_try.exe (125952 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\write_file_try.pdb (1150976 bytes)
//   F:\Rust\2ndeditionRE\Rust_Essentials\Chapters\Chapter 11 - Exploring the standard library\code\write_file_try.rs (1089 bytes)



