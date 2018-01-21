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
    err_move_indexed();
    move_indexed();
    err_use_after_move();
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

//       persons 
// -------------------------- stack
//        |*|4|3|
// --------|-----------------
//         |
//         |
//       |<------[0]----->|<------[2]----->|<------[3]----->| 
// --------|------------------------------------------------------- heap
//       | * | 8 | 5 |1988| * | 4 | 3 |1984| * | 8 | 5 |1990|
// --------|----------------|----------------|---------------------
//         |                |                |name        brith
// --------|-------     ----|-----       ----|----------
//      |alice|            |bob|            |molly|
// ----------------     ----------       ---------------
//
fn print_person() {
    struct Person { name: String, birth: i32 }
    let mut persons = Vec::new();
    persons.push(Person { name: "alice".to_string(), birth: 1988 });
    persons.push(Person { name:   "bob".to_string(), birth: 1984 });
    persons.push(Person { name: "molly".to_string(), birth: 1990 });
    for p in &persons {
        println!("{}, born {}", p.name, p.birth);
    }
}

//  in Rust, assignments of most types move the value from the source to the destination, leaving
//  the source uninitialized.
// 
//  before initializing t :                
//
//         s 
//  ------------------------- stack
//        |*|4|3| 
//  -------|-----------------
//         |
//  -------|--------------------------------------------- heap
//        |*| 8 | 5 |*| 4 | 3 |*| 8 | 5 |
//  -------|---------|---------|---------------------
//         |         |         |
//  -------|------ --|-----  --|----------
//        |alice|   |bob|     |molly|
//  -------------- --------  -------------
//                  
//  after initializing t :                
//
//         s (moved)        t  
//  ---------------------------------------- stack
//        | | | |         |*|4|3| 
//  -----------------------|----------------
//          +--------------+ 
//  --------|--------------------------------------------- heap
//         |*| 8 | 5 |*| 4 | 3 |*| 8 | 5 |
//  --------|---------|---------|---------------------
//          |         |         |
//  --------|------ --|-----  --|----------
//        |alice|   |bob|     |molly|
//  --------------- --------  -------------
//
#[allow(dead_code,unused_variables)]
fn err_moved_value() {
   let s = vec!["alice".to_string(), "bob".to_string(), "molly".to_string()];
   let t = s;  
   //let u = s;  // error : use of moved value 
}

//
//  after clone :                
//
//         s                 t(cloned)  
//  ---------------------------------------- stack
//     |*|4|3|               |*|4|3| 
//  ----|---------------------|----------------
//      |                     |
//  ----|---------------------|------------------------ heap
//     |*|8|5|*|4|3|*|8|5|   |*|8|5|*|4|3|*|8|5|
//  ----|-----|-----|---------|-----|-----|------------
//      |     |     |         |     |     |
//  ----|-----|-----|---------|-----|-----|------------ heap
//   |alice| |bob| |molly|  |alice| |bob| |molly|
//  --------------------------------------------------
//
#[allow(dead_code,unused_variables)]
fn clone_moved_value() {
   let s = vec!["alice".to_string(), "bob".to_string(), "molly".to_string()];
   let t = s.clone();
   let u = s.clone();
}

fn err_move_indexed() {
    let x = vec![10, 20, 30];
    let x1 = x[1];
    assert_eq!(20,x1);
	// Build a vector of the strings "101", "102", ... "105"
	let mut v = Vec::new();
	for i in 101 .. 106 {
		v.push(i.to_string());
	}
	// Pull out random elements from the vector.
	//let third = v[2];  //error : cannot move out of indexed content
}

fn move_indexed(){
	// Build a vector of the strings "101", "102", ... "105"
	let mut v = Vec::new();
	for i in 101 .. 106 {
		v.push(i.to_string());
	}

	// 1. Pop a value off the end of the vector:
	let fifth = v.pop().unwrap();
	assert_eq!(fifth, "105");

	// 2. Move a value out of the middle of the vector, and move the last
	// element into its spot:
	let second = v.swap_remove(1);
	assert_eq!(second, "102");

	// 3. Swap in another value for the one we're taking out:
	let third = std::mem::replace(&mut v[2], "substitute".to_string());
	assert_eq!(third, "103");

	// Let's see what's left of our vector.
	assert_eq!(v, vec!["101", "104", "substitute"]);
}

fn err_use_after_move(){

	let v = vec!["liberté".to_string(),
	"égalité".to_string(),
	"fraternité".to_string()];

	for mut s in v {  //value moved here
		s.push('!');
		println!("{}", s);
	}
	//v.push("a".to_string()); //error: value used after move

}
//  Using Rc, which holds a reference count and space for the String.
//
//           s    t    u
//  ----------------------------------- stack
//          |*|  |*|  |*|
//  ---------|----|----|---------------
//         +-+----+----+
//  -------|--------------------------- heap
//        |3| |*|16|9|
//  -----------|-----------------------
//             |
//  -----------|-------------
//          |shirataki|
//  -------------------------
//
use std::rc::Rc;
fn _using_rc(){
    // Rust can infer all these types; written out for clarity
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let _t: Rc<String> = s.clone();
    let _u: Rc<String> = s.clone();

}
//  Once you have become comfortable with both ownership and borrowing, you will have climbed the
//  steepest part of Rust’s learning curve, and you’ll be ready to take advantage of Rust’s unique
//  strengths.

