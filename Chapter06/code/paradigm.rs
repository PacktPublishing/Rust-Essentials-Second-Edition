fn main()  {
    let str1 = "abc";
    println!("{}", str1.len());     // 3
    println!("{}", str::len(str1)); // 3
}