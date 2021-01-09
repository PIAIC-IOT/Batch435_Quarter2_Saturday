fn main() {
    println!("Hello, world!");
    let first = 15 ;
    println!("Decimal {}",first);
    //println!("Binary  {}",first.bytes());
    println!("Binary {:b}",first);
    println!("Hex    {:x}",first);
    println!("Octal  {:o}",first);

    let one = 12;
    let two = 22;
    println!("one Decimal {} Binary  {:b}",one,one);
    println!("two Decimal {} Binary {:b}",two,two);
    let result = one & two;
    println!("and & Decimal {} Binary {:b}",result,result);
    let result = one | two;
    println!("OR  | Decimal {} Binary {:b}",result,result);
    let result = one ^ two;
    
    println!("one Decimal {} Binary  {:b}",one,one);
    println!("two Decimal {} Binary {:b}",two,two);
    println!("ZOR ^ Decimal {} Binary {:b}",result,result);
    println!("Not ! Decimal {} Binary {:b}  : Not {}",one,one,!one);
    let one:i8 = -13;
    println!("Not ! Decimal {} Binary {:b}  : Not {}",one,one,!one);
    let one :u8 = 12;
    

    println!("              Decimal {} Binary {:b} ",one,one);
    let result = one << 2;
    println!("Left shift << Decimal {} Binary {:b} ",result,result);
    let result = one << 6;
    println!("Left shift << Decimal {} Binary {:b} ",result,result);

    let result = one >> 2;
    println!("Right shift >> Decimal {} Binary {:b} ",result,result);
    let result = one >> 6;
    println!("Right shift >> Decimal {} Binary {:b} ",result,result);

    println!("Address / pointer {:p}",&result);
    let data = 0x_fff;
    println!("{}",data);

    // let name = "A";
    // let a = 65;
    // println!("one Decimal {} Binary  {:b}",a,a);

}

