


trait defaulttrait {
    fn message(&self)->String {
        format!("Due to COVID, We will remain CLOSED")
    }
}

trait GeneralOrder {
    fn order(&self)->String;
}

#[derive(Debug)]
struct Lunch {
    main : String,
    cold_drink : String,
    quantity : u8
}

#[derive(Debug)]
struct Dinner {
    main : String,
}



impl GeneralOrder for Lunch {
    fn order(&self)->String{
        format!("General Order {} {} {}",self.main,self.cold_drink,self.quantity)
    }
}

impl defaulttrait for Lunch {
    fn message(&self)->String{
        format!("Welcome at Ehsan Biryani Chain")
    }
}

impl defaulttrait for Nashta {}

impl defaulttrait for Dinner {}

impl Lunch {
    fn lunchorder(&self)-> String {
        format!("{} {} {}",self.main,self.cold_drink,self.quantity)
    }
}


#[derive(Debug)]
struct Nashta {
    main_course : String,
    hot_drink : String,
    quantity : u8
}

impl GeneralOrder for Nashta {
    fn order(&self)->String{
        format!("General Order : {} {} {}",self.main_course,self.hot_drink,self.quantity)
    }
}




impl Nashta {
    fn Nashtaorder(&self)-> String {
        format!("{} {} {}",self.main_course,self.hot_drink,self.quantity)
    }
}

fn main() {
    let hadia =  Nashta {
        main_course : String::from("Halwa Puri"),
        hot_drink : String::from("Milk"),
        quantity : 2
    };

    let ehsan = Lunch {
        main : String::from("Briyani"),
        cold_drink : String::from("7up"),
        quantity : 1
    };

    println!("Hadia's Nashta : {:#?}",hadia.Nashtaorder());
    println!("Ehsan's Lunch : {:#?}",ehsan.lunchorder());

    println!("Hadia's Nashta : {:#?}",hadia.order());
    println!("Ehsan's Lunch : {:#?}",ehsan.order());

    println!("Ehsan's Lunch : {:#?}",ehsan.message());
    println!("Hadia's Nashta : {:#?}",hadia.message());

    let affan = Dinner { main:String::from("Gouper") };
    println!("Affan's Dinner : {:#?}",affan.message());
    
    three(&ehsan);
    three(&hadia);
    three(&affan);
    // ehsan.message();
    printgeneric(55);
    printgeneric(String::from("Good Morning for Today session"));

    let rida = retun_trait();
    println!("{}",rida.order());

}

// fn three(data: &Lunch){
//     println!("we are in three function");
//     println!("{}",data.message());
// }

// fn three(data: &impl defaulttrait){
//     println!("we are in three function");
//     //println!("{} {} {}",data.main,data.quantity,data.quantity);
//     println!("{}",data.message());
// }


fn three <A: defaulttrait> (data: &A) {
    println!("we are in three function");
    //println!("{} {} {}",data.main,data.quantity,data.quantity);
    println!("{}",data.message());
}

fn printgeneric< T : std::fmt::Display > (mydata:T){
    println!("here we are printing {}",mydata);
}


fn retun_trait() -> impl GeneralOrder {
    println!("We are moving through return Trait");
    Nashta {
        main_course : String::from("Omlete + Paratha"),
        hot_drink : String::from("Karak Doodh Patti"),
        quantity : 1
    }
}