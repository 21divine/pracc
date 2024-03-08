# Numbers

### Integer

1. 🌟 

> Tips: If we don't explicitly assign a type to a variable, then the compiler will infer one for us.

// Remove something to make it work
fn main() {
    let x: i32 = 5;
    let mut y = 5; // y: i32 for standard integer type 

    y = x;
    
    let z: i32 = 10; // Type of z ? = i32 for standard integer type

    println!("Success!");
}

2. 🌟

// Fill the blank
fn main() 
{ 
    // 38_u8 -> v = 38 que tem o tipo "u8"; V tem o valor de 38 com o tipo de unsignd integer de 8 bits
    let v: u16 = 38_u8 as u16; // "as" convert an interger type to another interger type

    println!("Success!");
}

3. 🌟🌟🌟  

> Tips: If we don't explicitly assign a type to a variable, then the compiler will infer one for us.


// Modify `assert_eq!` to make it work
fn main() 
{
    let x: u32 = 5; // just changed i32 to u32
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String 
{
    format!("{}", std::any::type_name::<T>()) //i32
}


4. 🌟🌟 


// Fill the blanks to make it work
fn main() 
{
    assert_eq!(i8::MAX, 127); //MAX returns the largest number we can get in that integer type
    assert_eq!(u8::MAX, 255); 

    println!("Success!");
}


5. 🌟🌟 


// Fix errors and panics to make it work
fn main() {
   let v1 = 251_u8 + 4; // = 259, o MAX é 255. trocar o 8 por 4
   let v2 = u8::checked_add(251, 4).unwrap();
   println!("{},{}",v1,v2);
}

//2nd solution; change the u8 for a larger one: u16. same for the i8
fn main() 
{
    let v1: u16 = 251_u16 + 8; 
    let v2: i16 = i16::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
 }
6. 🌟🌟


// Modify `assert!` to make it work
fn main() 
// in Rust you can do math with different systems of numbers types
{ //1_024 - 1024. //0xff = 255 // 0o77 = 63 // 0b1111_1111 = 255
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255 = 1597
    assert!(v == 1597);

    println!("Success!");
}



### Floating-Point
7. 🌟

// Fill the blank to make it work
fn main() {
    let x: f64 = 1_000.000_1; // f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}


8. 🌟🌟 Make it work in two distinct ways

fn main() 
// problem on the floating point (f64) -> this is TOO PRECISE
{ // 1st solution
    assert!(0.1_f32+0.2_f32==0.3); // 0.1 + 0.2 is not EXACT 0.3; is something like that = 0.30000000000000001


    println!("Success!");
}
// 2nd solution

    assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32);


### Range
9. 🌟🌟 Two goals: 1. Modify `assert!` to make it work 2. Make `println!` output: 97 - 122


fn main() 
{
    let mut sum: i32 = 0; // sum = 0 e i = -3. sum = sum + i -> 0 + (-3) = -3 
    // 2nd interation -> -3 + (-2) = -5
    // 3rd interation -> -5 + (-1) = -6
    // 4st interation -> -6 + 0 = -6
    // 5st interation -> -6 + 1 = -5
    // sum termina valendo "-5"
    
    for i in -3..2 // for loop: i começa como -3 e o "for loop" será executado até atingir o valor de 1. o 2 ali é ignorado. "até 2" e não "igual a 2"
    // iterating to -3 to 1
    {
        sum += i
    }

    assert!(sum == -5);

    // digitar todas as letras do alfabeto; de A até Z
    for c in 'a'..='z' { // 'a'..='z' -> z está incluido, então o loop for não irá parar antes.
        println!("{}",c as u8); // colocar "c as u8" irá imprimir o valor na tabela ASCII correspondente à letra. a = 97, por exemplo;
    }
}


10. 🌟🌟 

// Fill the blanks
use std::ops::{Range, RangeInclusive};
fn main() 
{
    assert_eq!((1..5), Range{ start: 1, end: 5 }); // 1 até 5 -> 1..5
    assert_eq!((1..=5), RangeInclusive::new(1, 5)); // 1..=5 -> 5 está incluído. RangeInclusive

    println!("Success!");
}
                                                                                                                                                                                                                                                          

### Computations

11. 🌟 
```rust,editable

// Fill the blanks and fix the errors
fn main() {
    // Integer addition
    assert!(1u32 + 2u32 == 3u32);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2i8 == -1i8); 
    
    assert!(3 * 50 == 150);

    assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32);

    assert!(24 % 5 == __);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); // 0001
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);  // 0111
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); // 0110
    println!("1 << 5 is {}", 1u32 << 5); // 1 << 5 is 32 // "<<" is for bitwise left shiftting // 1 está pulando para a 5a casa "binária"
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); // is 0x20
}


> You can find the solutions [here](https://github.com/sunface/rust-by-practice/blob/master/solutions/basic-types/numbers.md)(under the solutions path), but only use it when you need it
