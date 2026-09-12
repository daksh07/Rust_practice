fn main() {
    println!("Hello, world!");
    let x: i8 = -2;
    let y: u8 = 34;
    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);

    //////////Arrays///////////////////
    let numbers: [i8;5] = [1,2,3,4,5];
    println!("Numbers Array: {:?}",numbers);
    let fruits: [&str; 3] = ["Apple", "Orange", "Grapes"];
    println!("Fruits Array: {:?}",fruits);
    println!("Fruit element 1: {}",fruits[0]);
    //////////////////////////////////
    
    /////////Tuples//////////////////
    let data: (String, i8, bool) = ("Simran".to_string(), 18, false);
    println!("Tuple example: {:?}",data);

    /////////String Slice/////////
    let full_string: String = String::from("Hello World");
    let string_slice: &str = &full_string[0..5];
    println!("Full string: {:?}",full_string);
    println!("String slice: {:?}",string_slice);
    
    /////////Function args and return///////
    let weight = 64.0;
    let height = 170.0;
    let bmi = calculat_bmi(weight, height);
    println!("Your BMI is {}",bmi);

    ////////Mutable and immutable/////////
    let mut x = 9.0;
    let y = x;
    println!("Value of x {}",x);
    println!("Value of y {}",y);
}

fn calculat_bmi(weight: f64, height: f64) -> f64{
    weight/(height * height)
}
