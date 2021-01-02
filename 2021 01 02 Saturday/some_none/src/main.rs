fn main() {
    let age = [25,39,24];
    //index    0   1  2;
    let zero = age.get(0);
    
    println!("Zero {:?}",zero);
    let ten = age.get(10);
    //let ten = age[10];
    let name = String::from("Ehsaan Rafiq");
    let name_length = name.len();

    println!("Ten {:?}",ten);
}
