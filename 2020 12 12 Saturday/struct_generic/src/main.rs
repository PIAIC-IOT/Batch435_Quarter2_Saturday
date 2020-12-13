#[derive(Debug)]
struct Nashta {
    main_course : String,
    hot_drink : String,
    quantity :u8,

}

#[derive(Debug)]
struct  Data <D,E,F> {
    roll_number : D,
    age : E,
    marks :F,
}

fn main() {
    let nashta1 = Nashta{
        main_course : "Egg + Cereal".to_string(),
        hot_drink   : "Tea".to_string(),
        quantity    : 1
    };

    println!("Zain Nashta {:#?}",nashta1);

    let nashta2 = Nashta{
        main_course : "Paratha + Gajar Halwa".to_string(),
        hot_drink   : "Milk".to_string(),
        quantity    : 1
    };
    println!("Kabeer Wala Nashta {:#?}",nashta2);

    let student_1 = Data{
        roll_number : "My roll number is one two three four five six",
        age         : 66.6,
        marks       : "My marks are ninety nine".to_string(),
    };
    println!("Student 1 Data {:#?}",student_1);
}