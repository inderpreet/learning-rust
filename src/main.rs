/**
 * Test Code for Rust Exercises
 */


// Entry Point method
fn main() {
    test_1();
    arrys();
    mdas();
    hello_array();
    matrix_test();
    vec_test();

    let _res = average_of_3(13, 2.3, 120.0);
    assert_eq!(_res, 45.1);

    celcius_to_farenheit(23.0);

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