//  1. The fn keyword (pronounced “fun”) introduces a function
//  2. the mut keyword (pronounced “mute”, short for mutable) By default,
//     once a variable is initialized, its value can’t be changed,
//  3. type u64, an unsigned 64-bit integer.
//  4. -> token precedes the return type
fn gcd(mut n: u64, mut m: u64) -> u64 {
    // 5. assert! macro, verifying that neither argument is zero.
    // 6. The ! character marks this as a macro invocation, not a function call.
    assert!(n != 0 && m != 0);
    // 7. does not require parentheses around the conditional expressions
    while m != 0 {
        if m < n {
            // 8. A let statement declares a local variable, don’t need to write out
            //    t’s type, as long as Rust can infer it  
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    // 9. If a function body ends with an expression that is not followed by a semicolon,
    // that’s the function’s return value.
    n
}

// 10. #[test] marks a test function, test_gcd() skipped in normal compilations, 
//     but included and called automatically with the 'cargo test' command.
// 11. #[test] is an attribute. like #ifdef in C and C++, or annotations in Java
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}


// 12. use declarations bring the two traits Write and FromStr 
// 13. a trait is a collection of methods that types can implement.
//     we never use the names Write or FromStr elsewhere in the program, 
//     a trait must be in scope in order to use its methods.
// 13.1 Write -> write_fmt -> std::io::Stderr   
// 13.2 FromStr -> from_str -> u64::from_str
use std::io::Write;
use std::str::FromStr;

// 15.  main function doesn’t return a value, so we can simply omit the ->
// 16.  and omit the parameter list.
fn main() {
    // 17.  Vec is Rust’s growable vector type, analogous to C++’s std::vector,
    //      a Python list, or a JavaScript array.
    // 17.1 mark the variable mut to allow us to push number onto it 
    // 17.2 need not write Vec<u64>, Rust will infer it
    let mut numbers = Vec::new();
    // 18.  for loop to process our command-line arguments
    // 19.  std::env::args function returns an iterator
    // 20.  the iterator’s skip method to produce a new iterator that omits that first value
    for arg in std::env::args().skip(1) {
        // 21.  u64::from_str to parse cmd-line arg as an unsigned 64-bit int
        // 22.  u64::from_str is a function associated with the u64 type, 
        //      akin to a static method in C++ or Java. 
        // 23.  from_str function doesn’t return a u64 directly, but rather a Result value
        // 23.1 A value written Ok(v), the parse succeeded and v is the value produced
        // 23.2 A value written Err(e), that the parse failed and e is an error why
        // 24.  Rust does not have exceptions: all errors are handled using either 
        //      Result or panic. Functions that perform input or output or otherwise 
        //      interact with the operating system all return Result types
        // 25.  check the success of our parse by using Result’s expect() method. 
        // 25.1 If Err(e), expect() prints a message of e, and exits program immediately
        // 25.2 if Ok(v),  expect() returns v itself, which we push onto vec
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }
    // 25. check at least one element, or exit the program with an error if it doesn’t
    if numbers.len() == 0 {
        // 26.  writeln! macro to write error msg 
        // 26.1 std::io::stderr() to stderr output stream
        // 26.2 unwrap() shortcut to check the print err msg did not itself fail
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    // 27.  & operator in &numbers[1..] borrows a reference to the vector’s elements 
    //      from the second onward.
    for m in &numbers[1..] {
        // 28.  The * operator in *m dereferences m, yielding the value it refers to
        d = gcd(d, *m);
    }
    // 29. println! macro takes a template string, substitutes arguments for the {...} 
    //     in the template string, and writes the result to the standard output stream.
    println!("The greatest common divisor of {:?} is {}", numbers, d);
    
    // 30.  Rust assumes that if main returns at all, the program finished successfully
    // 30.1 Unlike C and C++, main() return zero if finished successfully, or a nonzero
    //      exit status if something went wrong
    // 30.2 Only by explicitly calling like expect() or std::process::exit can cause 
    //      an error status code.
}
