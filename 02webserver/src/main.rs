//
//
// 1.  `extern crate` directives make crates that we cited in our Cargo.toml available.
// 2.  #[macro_use] attribute alert we plan to use macros exported by the crate.
extern crate iron;
#[macro_use] extern crate mime;

// 3.  iron::prelude::* makes all the public names of the iron::prelude module directly visible.
use iron::prelude::*;
use iron::status;

fn main() {

    println!("Serving on http://localhost:3000...");
    // 4. pass the get_form function to Iron::new, indicating that the server should use that
    //    function to handle all requests
    //Iron::new(get_form).http("localhost:3000").unwrap();

    build_router();
    
}

// 5. get_form function itself takes a mutable reference, written &mut, to a Request value
//    representing the HTTP request we’ve been called to handle.
// 6. _request parameter never be used, giving the parameter a name beginning with _ tells
//    Rust that we expect the variable to be unused, so it shouldn’t warn about.
fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    // 7.  The set_mut method uses its argument’s type to decide which part of the response to set
    // 7.1 status::Ok sets the HTTP status
    response.set_mut(status::Ok);
    // 7.2 media type (by mime! macro) sets Content-Type header
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    // 8.  Rust “raw string” syntax
    // 8.1 the letter r, zero or more hash marks (that is, the # character), a double quote
    // 8.2 then the contents of the string,
    // 8.3 terminated by another double quote followed by the same number of hash marks
    // 8.4 no escape sequences like \" are recognized
    // 8.5 We can always ensure the string ends where we intend by using more hash marks around the
    //     quotes than ever appear in the text
    response.set_mut(r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
          <input type="text" name="n"/>
          <input type="text" name="n"/>
          <button type="submit">Compute GCD</button>
        </form>
    "#);

    // 9.  IronResult<Response>, is another variant of the Result type
    // 9.1 Ok(r) for some successful Response value r, or Err(e) for some error value e.
    Ok(response)
}

//10.  Rust allows declarations to occur in any order
//10.1 Macro definitions and extern crate items with #[macro_use] attributes are exceptions to this
//     rule: they must appear before they are used.
extern crate router;
use router::Router;

fn build_router() {

    //11. create a Router, establish handler functions for two specific paths
    let mut router = Router::new();
    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    //12. pass this Router as the request handler to Iron::new
    //    consults the URL path to decide which handler function to call
    Iron::new(router).http("localhost:3000").unwrap();
}

extern crate urlencoded;

use std::str::FromStr;
use urlencoded::UrlEncodedBody;


fn post_gcd(request: &mut Request) -> IronResult<Response> {

	let mut response = Response::new();

    //13.  check `match` expression of a Result type 
    //13.1 if Err(e), it runs the branch with error set to e 
    //13.2 if Ok(v),  it runs the branch with success set to v, aka map -> form_data
    //14.  the program can only access the value of a Result by first checking which variant it is;
    //     one can never misinterpret a failure value as a successful completio
    //15.  ::<UrlEncodedBody> part of the method call is a type parameter indicating which part of
    //     the Request get_ref should retrieve.
    //16.  The format! macro uses the same kind of string template as the writeln! and println!
    //     macros, but returns a string value
	let form_data = match request.get_ref::<UrlEncodedBody>() {
		Err(e) => {
			response.set_mut(status::BadRequest);
			response.set_mut(format!("Error parsing form data: {:?}\n", e));
			return Ok(response);
		}
		Ok(map) => map
	};

	let unparsed_numbers = match form_data.get("n") {
		None => {
			response.set_mut(status::BadRequest);
			response.set_mut(format!("form data has no 'n' parameter\n"));
			return Ok(response);
		}
		Some(nums) => nums
	};

	let mut numbers = Vec::new();
	for unparsed in unparsed_numbers {
		match u64::from_str(&unparsed) {
			Err(_) => {
				response.set_mut(status::BadRequest);
				response.set_mut(
					format!("Value for 'n' parameter not a number: {:?}\n",
							unparsed));
				return Ok(response);
			}
			Ok(n) => { numbers.push(n); }
		}
	}

	let mut d = numbers[0];
	for m in &numbers[1..] {
		d = gcd(d, *m);
	}

	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut(
		format!("The greatest common divisor of the numbers {:?} is <b>{}</b>\n",
				numbers, d));
	Ok(response)
}


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
