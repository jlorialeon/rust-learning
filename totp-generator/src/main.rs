fn main() {
    println!("Hello, world!");
    let mut text = String::from("Original String");
    println!("{}",text);
    test(&mut text);
    
}

fn test(text:&mut String) -> (){
    *text = String::from("New string")
}
