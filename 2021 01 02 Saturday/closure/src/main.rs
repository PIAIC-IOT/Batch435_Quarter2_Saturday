

fn main() {
    println!("Welcome to Fourth ZOOM session");
    println!("PIAIC Batch 4-35 IOT Quartoer 2");

    let next_year=2022;
    year(2021);
    
    let my_closure = | | { 
        println!("Welcome to Closure");
    };
    
    my_closure();

    let closure_data = |age:u8| { 
        println!("Welcome to Closure Data {}",age);
    };
    
    closure_data(55);

    let closure_return = | |->u8 { 
        println!("Welcome to Closure Return ");
        66
    };
    
    let result = closure_return();
    println!("{}",result);

    let closure_plus = |data | { 
        println!("Welcome to Closure Plus ");
        data+1
    };
    
    println!("we got increment {}",closure_plus(77));
    
    let closure_next = |data| { 
        println!("Welcome to Closure Next {}",data);
    };
    
    
    closure_next(String::from("Fifty"));
    //closure_next(55);

    let salary = String::from("Ninety Nine Thousand");
    println!("Ehsaan Salary {}",salary);
    
    let closuer_environment = || {
        println!("Welcome to Environment Salary {}",salary);
    };
    
    closuer_environment();

    println!("Ehsaan Salary {}",salary);

}


fn year(data:u32) {
    println!("Happy New Christian Year {}",data);
    //println!("Next Year {}",next_year);
}