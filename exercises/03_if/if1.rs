fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    if a >b {
        println!("a:{} is bigger than b: {}", a,b);  
        a 
    }
    else if b >a {
    
        println!("b: {} is bigger than a: {}", b,a);   
        b
    }
    else{
        println!("they are equal");
        a //or b doesnt matter
    }
}

fn main() {
    // You can optionally experiment here.
    let tryit = bigger(56,89);
    println!("the bigger one is:{}", tryit);
}


// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
