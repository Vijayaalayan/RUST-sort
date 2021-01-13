fn main() {
    let mut number = vec![1, 5, 10, 2, 15];
    number.sort();
    println!("{:?}", number);
    
    let mut names = Vec::new();
    names.push("vijay");
    names.push("naveen");
    names.push("hariharan");
    names.push("nithin");
    names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    println!("{:?}", names);
    
}
