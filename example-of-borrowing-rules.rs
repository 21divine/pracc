let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s // no problem
let r3 = &s // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);

let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
println!("{} and {}", r1, r2);

let r3 = &mut s;
println!("{}", r3);