fn animal_habitat(animal: &str) -> &str {
    // TODO: Fix the compiler error in the statement below.
    let identifier = if animal == "crab" {
        //"Beach"
        1
    } else if animal == "gopher" {
        //"Burrow"
        2
    } else if animal == "snake" {
        //"Desert"
        3
    } else {
        //"Unknown"
        4
    };

    // Don't change the expression below!
    if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Dinasour"
    }
}

fn main() {
       
    let result1 = animal_habitat("gopher");
    let result2 = animal_habitat("snake");
    let result3 = animal_habitat("crab");
    let result4 = animal_habitat("g");
    
        
    println!("Result for 'gopher': {}", result1);  
    println!("Result for 'snake': {}", result2);  
    println!("Result for 'crab': {}", result3);  
    println!("Result for 'g': {}", result4);  
}
// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
