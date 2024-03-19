
1. 🌟 

fn main() 
{
    // Fill the blank with proper array type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Modify the code below to make it work
    assert!(arr.len() == 5);

    println!("Success!");
}

2. 🌟🌟


fn main() 
{
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3];
    let arr: [char; 3] = ['a', 'b', 'c'];
    
    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12); //3 x 4( each 'char' is 4 bytes in Rust)

    println!("Success!");
}


3. 🌟 All elements in an array can be initialized to the same value at once.
fn main() 
{
    // Fill the blank
    let list: [i32; 100] = [1; 100];

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}


4. 🌟 All elements in an array must be of the same type


fn main() 
{
    // Fix the error
    let _arr: [i32; 3] = [1, 2, 3];

    println!("Success!");
}

5. 🌟 Indexing starts at 0.


fn main() 
{
    let arr: [char; 3] = ['a', 'b', 'c'];
    
    let ele = arr[0]; // Only modify this line to make the code work!

    assert!(ele == 'a');

    println!("Success!");
}


6. 🌟 Out of bounds indexing causes `panic`.
// Fix the error
fn main() 
{
    let names: [String; 2] = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // But indexing is not safe
    let _name1 = &names[1];

    println!("Success!");
}


> You can find the solutions [here](https://github.com/sunface/rust-by-practice/blob/master/solutions/compound-types/array.md)(under the solutions path), but only use it when you need it