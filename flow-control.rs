# Flow control

### If/else
1. 🌟 


// Fill in the blanks
fn main() 
{
    let n: i32 = 5;

    if n < 0 
    {
        println!("{} is negative", n);
    } 
    
    else if n > 0 
    {
        println!("{} is positive", n);
    } 
    
    else
    {
        println!("{} is zero", n);
    }
} 


2. 🌟🌟 `If/else` expression can be used in assignments.


// Fix the errors
fn main() 
{
    let n: i32 = 5;

    let big_n: i32 =
        if n < 10 && n > -10 // 5 < 10 e 5 > -10
        {
            println!(", and is a small number, increase ten-fold");

            10 * n
        } 
        
        else 
        {
            println!(", and is a big number, halve the number");

            n / 2.0 as i32;
        };

    println!("{} -> {}", n, big_n);
} 


### For
3. 🌟 The `for in` construct can be used to iterate through an Iterator, e.g a range `a..b`.
fn main() 
{
    for n in 1..100 { // modify this line to make the code work // 1 vai até 99
        if n == 100 
        {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
} 



4. 🌟🌟 


// Fix the errors without adding or removing lines
fn main() 
{
    let names: [Strings; 2] = [String::from("liming"),String::from("hanmeimei")];
    for name in &names 
    {
        println!("{}", name);
    }

    println!("{:?}", names);

    let numbers: [i32; 3] = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers 
    {
        println!("{}", n);
    }
    
    println!("{:?}", numbers);
} 


5. 🌟

fn main() 
{
    let a: [i32; 4] = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i,v) in a.iter( ).enumerate()
    {
        println!("The {}th element is {}",i+1,v);
    }
}


### While
6. 🌟🌟 The `while` keyword can be used to run a loop when a condition is true.

// Fill in the blanks to make the last println! work !
fn main() 
{
    // A counter variable
    let mut n: i32 = 1;

    // Loop while the condition is true
    while n < 10 
    {
        if n % 15 == 0 
        {
            println!("fizzbuzz");
        } 
        
        else if n % 3 == 0 
        {
            println!("fizz");
        } 
        
        else if n % 5 == 0 
        {
            println!("buzz");
        } 
        
        else {
            println!("{}", n);
        }


        n += 1;
    }

    println!("n reached {}, so loop is over",n);
}
```

### Continue and break
7. 🌟 Use `break` to break the loop.


// Fill in the blank
fn main() 
{
    let mut n = 0;
    for i in 0..=100 {
       if n == 66 
       {
           break;
       }
       n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");


8. 🌟🌟 `continue` will skip over the remaining code in current iteration and go to the next iteration.


// Fill in the blanks
fn main() 
{
    let mut n = 0;
    for i in 0..=100 
    {
       
       if n != 66 
       {
           n+=1;
           continue;
       }
       
       break;
    }

    assert_eq!(n, 66);

    println!("Success!");
}


### Loop 

9. 🌟🌟 Loop is usually used together with `break` or `continue`.



// Fill in the blanks
fn main() {
    let mut count: u32 = 0_u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop 
    {
        count += 1;

        if count == 3 
        {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 
        {
            println!("OK, that's enough");

            break;
        }
    }
    // count in the end will hold a value of 5;
    assert_eq!(count, 5);

    println!("Success!");
}


10. 🌟🌟 Loop is an expression, so we can use it with `break` to return a value


// Fill in the blank
fn main() 
{
    let mut counter: i32 = 0;

    let result: i32 = loop 
    {
        counter += 1;

        if counter == 10 
        {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    println!("Success!");
}


11. 🌟🌟🌟 It's possible to break or continue outer loops when dealing with nested loops. In these cases, the loops must be annotated with some 'label, and the label must be passed to the break/continue statement.
// Fill in the blank
fn main() 
{
    let mut count = 0;
    'outer: loop {
        'inner1: loop 
        {
            if count >= 20 {
                // This would break only the inner1 loop
                break 'inner1; // `break` is also works.
            }
            count += 2;
        }

        count += 5;
        // count = 25
        'inner2: loop 
        {
            if count >= 30 {
                // This breaks the outer loop
                break 'outer;
            }

            // This will continue the outer loop
            continue 'outer;
        }
    }

    assert!(count == 30);

    println!("Success!");
}

> You can find the solutions [here](https://github.com/sunface/rust-by-practice)(under the solutions path), but only use it when you need it