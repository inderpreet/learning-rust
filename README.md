# RUST learning by doing

This repository contains my RUST learning and test Code

## Details
- IDE: CLion
- rustc: 1.66.0

## Notes

## Intro and basics

### Create a new rust project use
```bash
cargo new foo
```

### Comments are the same as in C++

### Print Macro

```rust
println!("Hello World");
print!("Hello World");

println!("Name: {}", name_var);
println!("{:03.1}'C is {}'F", celcius, farenheit);
// zero padded(if digits), 3 max digits in total and 1 decimal digit 

println!("{:?}", var2);
```

### variables
```rust
let var1 = 0;       // immutable
let mut var2 = 10;  // mutable

let var3: u8 = 10;
let var4: i32 = 40000;

let var5: f32 = 4.3;
let var6: bool = true;

let var7: char = 'Z';
```

### Operators and Operations

Most operators and operations are the same from C++

### Arrays (homogenous and fixed length)
```rust
let vowels = ['a', 'e', 'i', 'o', 'u'];
println!("First: {}", vowels[0]);

let numbers: [i32; 5]; // create an empty array of type i32 and size 5
numbers = [0; 5] // init with value 5 five times
let index: usize = numbers.len()-1; // special datatype
```

#### Multidimentional arrays

```rust
let matrix: [ [ [i32; 100]; 20]; 5]; // type size, type-size, type-size
matrix = [[[0; 100]; 20]; 5]; // init matrix

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
```

## Tuples (hetrogenous) - mutable or non-mutable

```rust
let a = (1, 0, '5', "Hello");
println!("First: {:?}", a.0);

let b:(u8, u8, char, &str);
```

## Functions

```rust
fn show_var(number: i32){
    println!("{:?}", number);
}
```

```rust
// Function to convert from celcius to F.
// takes one param of i64 and returns i64
fn celcius_to_farenheit(celcius: f64) -> f64{
    let farenheit = (celcius*1.8) + 32.0;
    println!("{:4.2}'C is {:?}'F", celcius, farenheit);
    return farenheit;
}
```


## Conditions

```rust
if x == 3{
    // do something here
} else if {
    // do something else here
} else {
    // last block
}
```

```rust
let x = if bool_var {1} else {2};
```

## Loops

```rust
loop{
    // do somethng infintely
    if condition_statement{
        break;
    }
}
```

```rust
// Loops that return a value
let mut x = 0;

let result = loop {
    if x == 10{
        break x * 10; // returns 100 and makes result=0
    }
    x+=1; // increment
}
```

```rust
// while loops
let mut x=0;
while x<10{
    count +=1; 
    // runs 10 times
}
```

```rust
// For loops

let vowels = ['a', 'e', 'i', 'o', 'u'];
for a in vowels{
    println!("{:?}", a);
}
```

```rust
// For loops
// old version of rust
let vowels = ['a', 'e', 'i', 'o', 'u'];
for a in vowels.iter() {
    println!("{:?}", a);
}
```

```rust
// For loops
// old version of rust
let vowels = ['a', 'e', 'i', 'o', 'u'];
for (index, item) in vowels.iter().enumerate() {
    println!("{:?} {:?}", index, item);
    if item == 'e'
        break;
}
```

```rust
// multi-dim loops
let matrix: [ [ [i32; 100]; 20]; 5]; // type size, type-size, type-size
matrix = [[[0; 100]; 20]; 5]; // init matrix

fn matrix_test() {
    let mut matrix = [[1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]];

    for row in matrix.iter_mut() { // mutable iterator
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
```

## Ownership

### Scope and shadowing

using let again to redefine the variable showdows the original var

## String literals and pointers in RUST

```rust
let str = String::from("My String");
// stored in a heap
// str is a pointer to the heap or the string
// contains the mem-loc, len, capacity
str.push_str(" and then.");
// adds to the allocated size and then changes its properties
```

## Cleanup memory - malloc and free; Grabage collection

integers live on the stack and string on heaps

```rust
let var1: String;
{
    let mut var2 = String::from("Value");
    var1 = var2; //invalidates var2 here
    // cannot access var2 here  user var2.clone() instead if needed. 
    // for integers this is not needed
}
println!("{:?}", var2);
```

## Resources

- https://doc.rust-lang.org/stable/book/
- 