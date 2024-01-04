/**
 * Test Code for Rust Exercises
 */


// Entry Point method
fn main() {
    println!("\n\n\n\n\n\n\n");
    // test_1();
    // arrys();
    // mdas();
    // hello_array();
    // matrix_test();
    // vec_test();

    // let _res = average_of_3(13, 2.3, 120.0);
    // assert_eq!(_res, 45.1);

    // celcius_to_farenheit(23.0);
    // ownership();

    // print_test();

    slice_test();

    println!("\n\n");

}

fn slice_test(){
    let message = String::from("Greetings from Earth");
	let first_word = get_first_word(&message[10..]);
	println!("First word is {}", first_word);
}

fn get_first_word(s: &str) -> &str {
// param type not &String
	let bytes = s.as_bytes();
	for(index, &item) in bytes.iter().enumerate() { // iterating over bytes 
		if item == b' ' {
			return &s[..index];
		}
	}
    &s
}

fn print_test(){
    let string1: &str = "This is a string.";
    let mut string2 = String::new() ;
    string2.push_str("This is a string 2.");
    let string3: &'static str = "This is a string 3";
    let string4 = String::from("Hello fellow Rustaceans!");

    println!("-------------------------");
    println!(" Print Test ");
    
    println!("{}", string1);
    println!("{:?} ", string2);
    println!("{}", string3);
    println!("{}", string4);

}


fn mdas() {
    let plt1 = [[1, 2, 3],
        [3, 4, 5]];
    let _arr3d = [[[0.0; 100]; 20]; 5];
    println!("Value of mdas: {}", plt1[0][2]);
}


fn test_1() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;
    let mut average: f64;
    average = a as f64 + b + c as f64;
    average = average / 3.0;

    assert_eq!(average, 45.1);
    println!("Test Passed");
}

fn arrys() {
    let mut letters = ['a', 'b', 'c'];
    let chara:&str = "Hello";
    println!("{:?}", chara);
    letters[0] = 'x';
    println!("First letter is {}", letters[0]);

    let nums: [i32; 5];
    nums = [0; 5];
    println!("Nums: {}", nums[4]);
}

fn hello_array() {
    let message = ['h', 'e', 'l', 'l', 'o'];

    for item in message {
        println!("items : {}", item);
    }
}

fn matrix_test() {
    let mut matrix = [[1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]];

    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
            print!("{}\t", num);
        }
        println!();
    }
    for _i in 1..20 {
        print!("-");
    }
    println!();

    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
            print!("{}\t", num);
        }
        println!("");
    }
}

fn vec_test() {
    let mut names: Vec<&str> = Vec::new();
    // let names = vec!["Inderpreet", "Gurpeet", "Anhad"];  // uses the rust macro
    //
    names.push("Inderpreet");
    names.push("Gurpreet");
    names.push("Anhad Makhni");

    for _name in names.iter() {
        println!("{}", _name);
    }

    println!("{}", names[2]);
}

fn average_of_3(a: i8,b:f64,c: f32) -> f64{
    let average:f64 = (a as f64 + b + c as f64) /3.0;
    println!("Average is {}", average);
    return average;
}


// Function to convert from celcius to F.
//
fn celcius_to_farenheit(celcius: f64)-> f64{
    let farenheit = (celcius*1.8) + 32.0;
    println!("{:4.2}'C is {}'F", celcius, farenheit);
    return farenheit;
}

fn ownership(){
    let var1: String;
    {
        let var2 = String::from("Value");
        var1 = var2;
        println!("Copying value");
    }
    println!("{:?}", var1);
}
