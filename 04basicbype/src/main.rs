extern crate regex;
use regex::Regex;
//
// Rust is a statically typed language: without actually running the program, the compiler checks
// that every possible path of execution will use values only in ways consistent with their types.
// This allows Rust to catch many programming mistakes early, and is crucial to Rust’s safety
// guarantees.
//

#[allow(dead_code)]
fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

// 1. Given the function’s return type, it’s obvious that v must be a Vec<i16>, a vector of 16-bit
//    signed integers; no other type would work.
// 2. Type inference gives back much of the legibility of dynamically typed languages, while still
//    catching type errors at compile time.
// 3. Rust’s generic functions give the language a degree of the same flexibility, while still
//    catching all type errors at compile time. (vs. Python and JavaScript, flexibile but difficult
//    to detect type errors early )
// 4. generic functions are just as efficient as their nongeneric counterparts
#[allow(dead_code)]
fn build_vector2() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}
#[test]
fn test_build_vector(){
    assert_eq!(Some(&10i16),build_vector().get(0));
    assert_eq!(Some(&10),   build_vector().get(0));
    assert_eq!(Some(&20i16),build_vector().get(1));

    assert_eq!(Some(&10),   build_vector2().get(0));
    assert_eq!(Some(&10i16),build_vector2().get(0));
}


fn main() {
    println!("basic types!");

    // 6. arithmetic operation overflowed
    //    let x = big_val + 1;
    //    wrap to a negative number
    let big_val = std::i32::MAX;
    let x = big_val.wrapping_add(1);  // ok
    assert_eq!( big_val,   2147483647);
    assert_eq!(       x,  -2147483648);

    // 7.  The prefixes 0x, 0o, and 0b designate hexadecimal, octal, and binary literals.
    assert_eq!(   0xcafeu32,  51966);
    assert_eq!(       0o106,     70);
    assert_eq!( 0b0010_1010,     42);

    // 8.  big number
    assert_eq!( 20_922_789_888_000u64, 20922789888000);

    // 9.  u8 byte literal
    assert_eq!(b'a', 97);
    assert_eq!(b'A', 65);
    assert_eq!(b'*', 42u8);

    // 10.  isize, usize, the same size as an address on the machine
    assert_eq!(137isize,  137);
    assert_eq!(137usize,  137);
    assert_eq!(-0b0101_0010isize, -82);
    assert_eq!( 0xffff_fc00usize, 4_294_966_272);

    // 11.  limitations
    assert_eq!(    std::i8::MAX,                  127);
    assert_eq!(    std::i8::MIN,                 -128);
    assert_eq!(   std::i16::MAX,                32767);
    assert_eq!(   std::i16::MIN,               -32768);
    assert_eq!(   std::i32::MAX,           2147483647);
    assert_eq!(   std::i32::MIN,          -2147483648);
    assert_eq!(   std::i64::MAX,  9223372036854775807);
    assert_eq!(   std::i64::MIN, -9223372036854775808);
    assert_eq!( std::isize::MAX,  9223372036854775807);
    assert_eq!( std::isize::MIN, -9223372036854775808);

    assert_eq!(    std::u8::MAX,           (127<<1)+1); //255
    assert_eq!(    std::u8::MIN,                    0);
    assert_eq!(   std::u16::MAX,         (32767<<1)+1); //65,535
    assert_eq!(   std::u16::MIN,                    0);
    assert_eq!(   std::u32::MAX,    (2147483647<<1)+1); //4,294,967,295
    assert_eq!(   std::u32::MIN,                    0);
    assert_eq!(   std::u64::MAX, 18446744073709551615);
    assert_eq!(   std::u64::MIN,                    0);
    assert_eq!( std::usize::MAX, 18446744073709551615);
    assert_eq!( std::usize::MIN,                    0);


    // 12.  Characters require a backslash
    assert_eq!(b'\'', 39   ); // Single quote, '
    assert_eq!(b'\'', 0x27 );
    assert_eq!(b'\\', 92   ); // Backslash, \
    assert_eq!(b'\\', 0x5c );
    assert_eq!(b'\n', 10   ); // Newline
    assert_eq!(b'\n', 0x0a );
    assert_eq!(b'\r', 13   ); // Carriage return
    assert_eq!(b'\r', 0x0d );
    assert_eq!(b'\t', 9    ); // Tab
    assert_eq!(b'\t', 0x09 );


    // 13.  Type Casts
    assert_eq!(   10_i8  as u16,    10_u16); // in range
    assert_eq!( 2525_u16 as i16,  2525_i16); // in range

    assert_eq!(   -1_i16 as i32,    -1_i32); // sign-extended
    assert_eq!(65535_u16 as i32, 65535_i32); // zero-extended

    // 13.1 truncation when type casting
    //      Conversions that are out of range for the destination
    //      produce values that are equivalent to the original modulo 2^N,
    //      where N is the width of the destination in bits. This
    //      is sometimes called "truncation".
    // 13.2 the act of writing out numeric conversions in Rust has
    //      alerted us to problems we would otherwise have missed.
    assert_eq!( 1000_i16 as  u8,   232_u8);
    assert_eq!(65535_u32 as i16,    -1_i16);

    assert_eq!(   -1_i8  as u8,    255_u8);
    assert_eq!(  255_u8  as i8,     -1_i8);

    // 14.  The standard library provides some basic operations
    //      for the basic types
    assert_eq!(2u16.pow(4), 16);            // exponentiation
    assert_eq!((-4i32).abs(), 4);           // absolute value
    assert_eq!(0b101101u8.count_ones(), 4); // population count

    assert_eq!(2_usize.pow(32), 4294967296);
    assert_eq!(2_usize.pow(32), 2_usize<<31);
    assert_eq!(std::usize::MAX, (((2_usize<<62)-1)<<1)+1);
    assert_eq!(2_u64.pow(32),   2_u64<<31);

    assert_eq!(std::u64::MAX.count_ones(),64);
    assert_eq!(std::i64::MAX.count_ones(),63);
    assert_eq!(std::i64::MAX, 0x7fff_ffff_ffff_ffff_i64);
    assert_eq!(0xff,0b1111_1111);
    assert_eq!(0x7f,0b0111_1111);

    // 15.  Foating-Point Type
    //      Rust provides IEEE single/double-precision floating-point types.
    //      Following the IEEE 754-2008 specification
    // 15.1 IEEE single precision (at least 6 decimal digits)
    // 15.2 IEEE double precision (at least 15 decimal digits)
    // 15.3 If a floating-point literal lacks a type suffix, Rust infers whether
    //      it is an f32 or f64 from the context, defaulting to f64.

    assert_eq!(1.61803f32, 1.61803);
    assert_eq!(6.0221e23f64, 6.0221e23);

    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.); // exactly 5.0, per IEEE
    assert_eq!(-1.01f64.floor(), -1.0);
    assert!((-1. / std::f32::INFINITY).is_sign_negative());

    // The standard library’s std::f32 and std::f64 modules define constants for
    // the IEEE-required special values like INFINITY,
    // NEG_INFINITY (negative infinity), NAN (the not-a-number value),
    // and MIN and MAX (the largest and smallest finite values).
    assert_eq!(std::f32::MIN, -3.4028235_e38_f32);
    assert_eq!(std::f32::MAX,  3.4028235_e38_f32);
    assert_eq!(1./std::f32::INFINITY, 0.);
    assert_eq!(1./std::f32::NEG_INFINITY, -0.);
    assert_eq!((2.0_f32).sqrt(),1.4142135);
    assert_eq!(f64::sqrt(2.0),1.4142135623730951);

    // 16.  bool
    // 16.1 as operator can convert bool values to integer types
    // 16.2 However, as won’t convert in the other direction, from numeric types to bool.
    assert_eq!(false as i32, 0);
    assert_eq!(true  as i32, 1);
    assert_eq!(false as u8, 0);
    assert_eq!(true  as u8, 1);

    // 17.  Unicode character
    assert_eq!('ಠ','\u{CA0}');
    assert_eq!('A','\x41');
    assert_eq!('字', '\u{5B57}');
    assert_eq!('*' as i32, 42);
    assert_eq!('ಠ' as u16, 0xca0);
    assert_eq!('ಠ' as i8, -0x60);

    // 17.1 The standard library provides some useful methods on characters
    //      from the module “std::char”.
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('Β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!('ಠ'.len_utf8(), 3);
    assert_eq!(std::char::from_digit(2, 10), Some('2'));

    // 18.  A tuple is a pair, or triple, or quadruple, ... of values of assorted types.
    // 18.1 Tuples aren’t much like arrays: for one thing, each element of a tuple can have a
    //      different type,
    assert_eq!(('a',b'A',0x61),('\x61',65,97)); //mixed types allowed

    // 18.2 Rust code often uses tuple types to return multiple values from a function
    //      the split_at method on string slices, which divides a string into two halves and
    //      returns them both
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    // 18.3 Given a tuple value t, you can access its elements as t.0, t.1, and so on.
    //      tuples allow only constants as indices, like t.4. You can’t write t.i or t[i]
    //      to get the i’th element.
    let line = "good or evil";
    let temp = line.split_at(5);
    let begin = temp.0;
    let end = temp.1;
    assert_eq!(begin, "good ");
    assert_eq!(end, "or evil");

    // 18.4 tuples used as a sort of minimal-drama struct type. We could declare a struct
    //      but that’s pretty heavy notation for something so obvious, so we just used a
    //      tuple
    // 18.5 unit (empty) tuple
    //      The other commonly used tuple type, is the zero-tuple (). This is traditionally
    //      called the unit type because it has only one value, also written (). Rust uses
    //      the unit type where there’s no meaningful value to carry, but context requires
    //      some sort of type nonetheless.
    assert_eq!((),());

    // 18.6 you may include a comma after a tuple’s last element: the types (&str, i32,) and
    //      (&str, i32) are equivalent)
    assert_eq!(("Brazil", 1985,),("Brazil",1985));
    // here is a tuple containing a single string; its type is (&str,)
    // missing the comma caused a mismatched type &str
    // assert_eq!(("lonely hearts",),("lonely hearts")); //mismatched types
    assert_eq!(("lonely hearts",),("lonely hearts",));

    // 19.  Pointer Types
    // 19.1 A value of type &String (pronounced “ref String”) is a reference to a String
    //      value, a &i32 is a reference to an i32, and so on.
    //      A reference can point to any value anywhere, stack or heap. The expression &x
    //      produces a reference to x; we say that it borrows a reference to x.
    // 19.2 And like a C pointer, a reference does not automatically free any resources
    //      when it goes out of scope.
    // 19.3 Unlike C pointers, Rust references are never null: there is simply no way to
    //      produce a null reference in safe Rust.
    // 19.4 Rust references are immutable by default,
    //      &T     is immutable reference, like const T* in C.
    //      &mut T is   mutable reference, like       T* in C.
    // 19.5 Rust has the raw pointer types *mut T and *const T. you may only dereference
    //      raw pointers within an unsafe block. An unsafe block is Rust’s opt-in mechanism
    //      for advanced language features whose safety is up to you.

    // 20.  Boxes
    //      Box::new() allocates enough memory to contain the tuple on the heap. When b
    //      goes out of scope, the memory is freed immediately, unless b has been moved
    //      by returning it.
    let t = (12, "eggs");
    let b = Box::new(t);                   // allocate a tuple in the heap
    assert_eq!(b,Box::new((12,"eggs")));

    // 21.  Array
    //      The type [T; N] represents an array of N values, each of type T. An array’s
    //      size is a constant determined at compile time, and is part of the type; you
    //      can’t append new elements, or shrink an array.
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);
    // 21.1 For the common case of a long array filled with some value, you can write
    //      [V; N], where V is the value each element should have, and N is the length.
    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    assert!(sieve[211]);
    assert!(!sieve[9867]);

    /* print out the primes in the range from 2..10000
       let mut count=0;
       for i in 2..10000 {
       if sieve[i] {
       count+=1;
       print!("{:5},",i);
       if count % 10 == 0 {
       println!()
       }
       }
       }
       println!("\n {} primes exist in the range of 2..10000",count);
       */

    // 22.2 use slice method on an array directly
    //      Rust implicitly converts a reference to an array to a slice when searching
    //      for methods, so you can call any slice method on an array directly. the
    //      sort method is actually defined on slices, but since sort takes its operand
    //      by reference, we can use it directly on chaos: the call implicitly produces
    //      a &mut [i32] slice referring to the entire array.
    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);

    // 23.  A vector Vec<T> is a resizable array of elements of type T, allocated on the
    //      heap.
    // 23.1 create vectors. the simplest is to use the vec! macro
    let mut v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);
    v.push(11);
    v.push(13);
    // |argument| { ... } is a Rust closure expression, the { } can ommit if closure
    // is one-liner
    assert_eq!(v.iter().fold(1, |a, b| a * b ), 30030);
    assert_eq!(v.iter().fold(1, |a, b|{a * b}), 30030);
    // 2*((2*(1+2)-1)+3)-1 = 15
    assert_eq!([2,3].iter().fold(1, |a, b|{let i = a + b; 2*i-1}), 15);
    // build a vector by repeating a given value a certain number of times
    assert_eq!(new_pixel_buffer(2,3),vec![0,0,0,0,0,0]);

    // 23.2 Vec::new is equivalent to calling vec! macro
    let mut v = Vec::new();
    v.push("step");
    v.push("on");
    v.push("no");
    v.push("pets");
    assert_eq!(v, vec!["step", "on", "no", "pets"]);

    // 23.3 build a vector from the values produced by an iterator
    let v: Vec<i32>  = (0..5).collect();
    // need to supply the type when using collect
    // let v = (0..5).collect();  -> cannot infer type
    assert_eq!(v, [0, 1, 2, 3, 4]);


    // 23.4 use slice methods on vectors
    let mut word = vec!["good","bad","ugly"];
    // the reverse() method is actually defined on slices, but the call implicitly borrows
    // a &mut [&str] slice from the vector, and invokes reverse()
    word.reverse();
    assert_eq!(word, ["ugly","bad", "good"]);
    word.sort();
    assert_eq!(word, ["bad","good", "ugly"]);

    // 23.4 Vector internal
    // A Vec<T> consists of three values:
    // - a pointer to the heap-allocated buffer allocated to hold the elements (pointer)
    // - the number of elements that buffer has the capacity to store (capacity)
    // - the number it actually contains now (its length)
    // When the buffer has reached its capacity, adding another element to the vector entails
    // allocating a larger buffer, copying the present contents into it, updating the vector’s
    // pointer and capacity to describe the new buffer, and finally freeing the old one.

    // you can call Vec::with_capacity to create a vector with a buffer large
    // enough to hold them all, right from the start
    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 4);

    // 23.5 insert & remove
    let mut v = vec![10, 20, 30, 40, 50];
    // Make the element at index 3 be 35.
    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);
    // Remove the element at index 1.
    v.remove(1);
    assert_eq!(v, [10, 30, 35, 40, 50]);

    let mut v = vec!["one", "two",];
    assert_eq!(v.pop(), Some("two"));
    v.push("three");
    assert_eq!(v.pop(), Some("three")); //LIFO
    assert_eq!(v.pop(), Some("one"));
    assert_eq!(v.pop(), None);

    let mut v = vec!["one", "two",];
    v.push("three");
    let mut a:Vec<_> = v.iter().skip(1).collect();
    assert_eq!(a,vec![&"two",&"three"]);
    a.push(&"four");
    assert_eq!(a,vec![&"two",&"three", &"four"]);

    let b:Vec<String> = v.iter().map(|s|s.to_string()).skip(1).collect();
    assert_eq!(b,["two","three"]);
    assert_eq!(b,["two","three"]);

    let mut v = Vec::new();
    v.push(String::from("1"));
    v.push(String::from("2"));
    v.push(String::from("3"));
    let a:Vec<_> = v.iter().skip(1).collect();
    assert_eq!(a,["2","3"]);
    let b:Vec<String> = v.iter().map(|s|s.to_string()).skip(1).collect();
    assert_eq!(b,["2","3"]);

    let s = ["one", "two", "three"];
    let a:Vec<_> = s.iter().skip(1).collect();
    let b:Vec<String> = s.iter().skip(1).map(|s|s.to_string()).collect();
    assert_eq!(a,[&"two",&"three"]);
    assert_eq!(b,["two","three"]);

    let s = [String::from("1"),String::from("2")];
    let a:Vec<_> = s.iter().collect();
    let b:Vec<String> = s.iter().map(|s|s.to_string()).collect();
    assert_eq!(a,["1","2"]);
    assert_eq!(b,["1","2"]);


    // 23.6  use a for loop to iterate over a vector
    let v = ["input","1","2","3","4","5"];
    let numbers: Vec<String> = v.iter().skip(1).map(|s|s.to_string()).collect();
    for num_str in numbers {
        let num = num_str.parse::<i32>().unwrap();
        println!("{}: {}", num,
                 if num % 2 == 0 {
                     "even"
                 } else {
                     "odd"
                 });
    }

    // 24.  Slices
    // 24.1
    // automatically converts the &Vec<f64> reference and the &[f64; 4] reference to slice
    // references that point directly to the data.
    //
    //    v                     a                 sa       sv
    // ------------------------------------------------------------
    // |*|4|4|      |0.0|0.456|1.0|0.456|        |&|4|    |&|4|
    // -|-------------|---------------------------|--------|-------  stack
    //  |             |                           |        |
    //  +-(1)-+       +---------(2)---------------+        |
    //        | +-----------------------(3)----------------+
    //        | |
    // -------|-|--------------------------------------------------  heap
    //       |0.0|0.123|1.0|0.123|
    // ------------------------------------------------------------
    //
    // (1) Pointer to vector v in heap (owned by itself)
    // (2) Reference to array a in stack (non-owning)
    // (3) Reference to vector v in heap (non-owning)

    let v: Vec<f64> = vec![0.0,  0.123,  1.0,  0.123];
    let a: [f64; 4] =     [0.0,  0.456,  1.0,  0.456];
    let sa: &[f64] = &a;
    let sv: &[f64] = &v;
    print(&sv);
    print(&sa);

    // 25.  String
    // 25.1 String Literals
    //      If one line of a string ends with a backslash, then the newline character and the
    //      leading whitespace on the next line are dropped:
    println!("On the 24th of February, 1815, the look-out at Notre-Dame de\
        la Garde signalled the three-\
        master, the Pharaon from Smyrna, Trieste, and Naples.");
    // 25.2 raw strings need not warry about escape sequences
    //  raw string using r
    let default_mac_install_path = r"~/Library/Application Support/";
    let pattern = Regex::new(r"\d+(\.\d+)*");
    println!("{}",default_mac_install_path);
    println!("{:?}",pattern);

    //  raw string using ###
    println!(r###"
         This raw string started with 'r###"'.
         Therefore it does not end until we reach a quote mark ('"')
         followed immediately by three pound signs ('###'):
    "###);

    // 25.2 Byte Strings
    let abc = b"ABC";
    assert_eq!(abc, &[b'A', b'B', b'C']);

    // 25.3 String, &str, and str
    //
    // alex:String  lex:&str(string slice)      wu:&str(string literal)
    // ------------------------------------------------------------
    // |*|4|4|       |*|3|                       |*|2|
    // -|-------------|---------------------------|----------------  stack
    //  |             |                          (3)
    //  +-(1)-+ +-(2)-+                 ----------|----------------  read-only memory
    //        | |                                |W|U|
    //        | |                       ---------------------------  preallocated
    // -------|-|-------------  heap
    //       |a|l|e|x|
    // -----------------------
    //
    // (1) String is a Vec<u8>
    // (2) &str (“string slice”) is a reference to str owned by someone else
    // (3) string literal is a &str that refers to preallocated text,

    // A String has a resizable buffer holding UTF-8 text.
    // The buffer is allocated on the heap, it can resize its buffer as needed or requested.
    // You can think of a String as a Vec<u8>
    let alex = "alex".to_string();
    let lex = &alex[1..];
    let wu = "WU";

    assert_eq!(alex,"alex");
    assert_eq!((lex,wu),("lex","WU",));

    assert_eq!("WU".len(), 2);  //2 bytes UTF8
    assert_eq!("WU".chars().count(), 2);

    assert_eq!("吴".len(), 3);  //3 bytes UTF8
    assert_eq!("吴".chars().count(), 1);

    // 25.4  modify
    // impossible to modify a &str
    // let mut s1 = "hello"
    // s1[0] = 'C'; // error: cannot be mutably indexed by `{integer}`
    let mut s1 = "hello".to_string();

    assert!(s1.get_mut(0..1).is_some());
    assert_eq!(s1.get_mut(0..1).map(|v|&*v),Some("h"));
    if let Some(s) = s1.get_mut(0..1) {
        assert_eq!(s,"h");
        s.make_ascii_uppercase();
    }

    assert_eq!(s1,"Hello");

    /*
    let s1m: &mut str = &mut s1.to_string();
    let me = unsafe { s1m.as_bytes_mut() };
    me.make_ascii_uppercase();
    println!("{:?}",me.iter().map(|x|*x as char).collect::<Vec<_>>());
    */

    let mut s2 = "hello".to_string();
    s2.push('a');
    println!("{}",s2);

    // &str is very much like &[T]: a fat pointer to some data.
    // String is analogous to Vec<T>:
    let s3 = "hello";
    assert_eq!("olleh",s3.chars().rev().collect::<String>());

    // The .to_string() method converts a &str to a String
    let err_msg = "unknown input type".to_string();
    assert_eq!("type input unknown",
       err_msg.split_whitespace().rev().map(|s| s.to_owned()+" ").collect::<String>().trim());

    // The format!() macro returns a new formatted String
    assert_eq!(format!("{}°{:02}′{:02}″N", 24, 5, 23),
        "24°05′23″N".to_string());

    //
    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");

    assert!("ONE".to_lowercase() == "one");
    assert!("peanut".contains("nut"));
    assert_eq!("🗻∈🌏".replace("🗻", "🍔"), "🍔∈🌏");


}

fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}

fn print(n: &[f64]) {
    for elt in n {
        print!("{:.3} ", elt);
    }
    println!()
}



