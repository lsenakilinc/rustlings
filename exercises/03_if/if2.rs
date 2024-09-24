// TODO: Fix the compiler error on this function.
fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } 
    else if fizzish == "fuzz" {
        "bar"
    } else {
        "bar"
    }
}

fn main() {
    
    let result1 = foo_if_fizz("fizz");
    let result2 = foo_if_fizz("fuzz");
    let result3 = foo_if_fizz("something else");

   
    println!("Result for 'fizz': {}", result1);  
    println!("Result for 'fuzz': {}", result2);  
    println!("Result for 'something else': {}", result3);  
}

// TODO: Read the tests to understand the desired behavior.
// Make all tests pass without changing them.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        // This means that calling `foo_if_fizz` with the argument "fizz" should return "foo".
        assert_eq!(foo_if_fizz("fizz"), "foo");
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar");
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz");
    }
}
