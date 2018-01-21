//
// Rust isn’t the kind of language you can learn in a couple days and just deal with the
// hard/technical/good-practice stuff later. You will be forced to learn strict safety 
// immediately and it will probably feel uncomfortable at first.
// -- Mitchell Nordine
//
// Two Promise: 
//
// * You (need to) decide the lifetime of each value in your program.
// * Program never use a pointer to an object after it has been freed. 
//
// Note: 
//
// 1.) Plenty of languages fulfill the second promise using garbage collection, 
// automatically freeing objects only when all reachable pointers to them are gone. 
//
// 2.) Rust in a way by restricting how your programs can use pointers.
//
// 3.) At runtime, your pointers are simple addresses in memory, just as they would be 
// in C and C++. The difference is that your code has been proven to use them safely.
//
//
fn main() {
    println!("Hello, Ownership!");
	print_padovan();
    print_person();
}

// In Rust, every value has a single owner that determines its lifetime.
// :w


// Padovan sequence (https://en.wikipedia.org/wiki/Padovan_sequence)
// (1) P(0)=P(1)=P(2)=1 
// (2) P(n)=P(n-2)+P(n-3)
//
//
// ------------------------- stack
//        |*|16|10|
// --------|----------------
//         |
//         |
//         |
//        |<---length(10)---->|
// --------|---------------------------------------- heap
//        |1|1|1|2|2|3|4|5|7|9| | | | | | |
// -------------------------------------------------
//        |<--------capacity(16)--------->|
//
//   (A Vec 32 on the stack, pointing to its buffer in the heap)
//
fn print_padovan() {
    let mut padovan = vec![1,1,1];  // allocated here
    for i in 3..10 {
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
}                                   // dropped here

//
// --------------------------
//
fn print_person() {
    struct Person { name: String, birth: i32 }
    let mut composers = Vec::new();
    composers.push(Person { name: "alice".to_string(), birth: 1988 });
    composers.push(Person { name:   "bob".to_string(), birth: 1984 });
    composers.push(Person { name: "molly".to_string(), birth: 1990 });
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
}


//  Once you have become comfortable with both ownership and borrowing, you will have climbed the
//  steepest part of Rust’s learning curve, and you’ll be ready to take advantage of Rust’s unique
//  strengths.
