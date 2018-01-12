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

    // 22.2 Rust implicitly converts a reference to an array to a slice when searching
    //      for methods, so you can call any slice method on an array directly. the
    //      sort method is actually defined on slices, but since sort takes its operand
    //      by reference, we can use it directly on chaos: the call implicitly produces
    //      a &mut [i32] slice referring to the entire array.
    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);

    // 23.  A vector Vec<T> is a resizable array of elements of type T, allocated on the
    //      heap.
    // 23.1 There are several ways to create vectors. The simplest is to use the vec! macro
    let mut v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);
    v.push(11);
    v.push(13);
    assert_eq!(v.iter().fold(1, |a, b| a * b), 30030);

}
