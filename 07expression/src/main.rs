fn main() {
    println!("Hello, expression!");
    // 1. Expression Language
    // 1.1 In Rust, if and match can produce values.
    // 1.2 Most of the control flow tools in C are statements. 
    // In Rust, they are all expressions.
    // 2. Blocks and Semicolons
    // 2.1 Blocks, too, are expressions. A block produces a value 
    // and can be used anywhere a value is needed
    // 2.2 In Rust, the semicolon actually means something.
    // when you leave the semicolon off the last line of a block, 
    // you’re making that block produce a value
    // Declarations
    // 3. Declarations
    // 3.1 A block can also contain item declarations. An item is 
    // simply any declaration that could appear globally in a program
    // or module, such as a fn, struct, or use.
    // 3.2 A nested fn cannot access local variables or arguments in 
    // the  scope
    // 4. match & if
    // match expressions are something like the C switch statement, but more flexible.
    let mut args:Vec<String> = std::env::args().skip(1).collect();
    match args.pop() {
        Some( code_str) => {
            let code:i32 = match code_str.parse::<i32>()
            {
                Ok(n) => n,
                Err(err) => {
                    println!("err {}",err);
                    -1
                }, 
                
            };
            match code {
                0 => println!("OK"),
                1 => println!("Wires Tangled"),
                2 => println!("User Asleep"),
                _ => println!("Unrecognized Error {}", code)
            }
        },
        None => {
            println!("None input");
        }
    }
    // 4.1 Rust prohibits match expressions that do not cover all possible values:
    // 4.2 All blocks of an if expression must produce values of the same type
    // 4.3 all arms of a match expression must have the same type 
    //
    // 5. if let
    // match can do everything if let can do. An if let expression is shorthand for a match with
    // just one pattern:
    // match expr {
    //     pattern => { block1 }
    //     _ => { block2 }
    // }
    //
    // 6. loop
    // 6.1 A while loop behaves exactly like the C equivalent, except that again, the condition
    //   must be of the exact type bool
    // 6.2 The while let loop is analogous to if let.
    // 6.3 Use loop to write infinite loops
    // 6.4 A for loop evaluates the collection expression
    // The .. operator produces a range, a simple struct with two fields: start and end. 0..20 is
    // the same as std::ops::Range { start: 0, end: 20 }. 
    for i in 0..20 {
        println!("{}", i);  // the last number printed is 19.
    }
    
    // In keeping with Rust’s move semantics, loop over a reference to the collection instead. 
    let mut strings: Vec<String> = Vec::new(); 
    strings.push("err1".to_string());
    strings.push("err2".to_string());
    //
    // for s in strings {                      // each String is moved into s here...
    //    println!("{}", s);
    // }                                       // ...and dropped here
    // println!("{} error(s)", strings.len()); // error: use of moved value
    for s in &strings {
        println!("{}", s);
    }
    println!("{} error(s)", strings.len());

}

