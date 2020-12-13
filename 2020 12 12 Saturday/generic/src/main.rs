fn main() {
    println!("    Hello, AIOT-Quarter 2  !");
    let receive = utype(35);
    println!("    Returned u8 Value        {}",receive);
    let sreceive = stype(String::from("Syed Salman Alam"));
    println!("    Returned String Value        {}",sreceive);
    let greceive = gtype(35);
    println!("    Returned Generic Value        {}",greceive);
    let greceive = gtype("Yougurt + Cereal");
    println!("    Returned Generic Value        {}",greceive);
    let greceive = gtype(78319.78319);
    println!("    Returned Generic Value        {}",greceive);
    let greceive = gtype(true);
    println!("    Returned Generic Value        {}",greceive);
    let greceive = gtype(("Muhammad Tahir Nazeer",160330,5.8));
    println!("    Returned Generic Value        {:?}",greceive);

}

fn utype(udata:u8)->u8 {
    udata
}

fn stype(sdata:String)-> String{
    sdata
}

fn gtype <I> (gdata:I)-> I {
    gdata
}

