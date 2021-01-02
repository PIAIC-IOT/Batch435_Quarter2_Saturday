struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(data: T) -> Cacher<T> {
        Cacher {
            calculation:data,
            value: None,
        }
    }

    fn value_method(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


fn main() {
    let mut myinstance = Cacher::new(|num| {
        num
    });

    println!("in start ---->  {:?}",myinstance.value);
    myinstance.value_method(22);
    println!("after work ---->  {:?}",myinstance.value);
    myinstance.value_method(55);
    println!("after work ---->  {:?}",myinstance.value);
    

}
