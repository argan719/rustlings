fn animal_habitat(animal: &str) -> &str {
    // TODO: Fix the compiler error in the statement below.
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        0
    };  // 여기에 세미콜론을 붙여야 하는 이유:
    // 여기서는 반환하지 않음. identifier에 '할당'하는 거임.
    // 반환은 밑에 if문에서 이루어지고 여기서는 표현식이 되어야 하므로 세미콜론을 붙여주는 것이 맞음.
    // 반대로 밑에 if문에서는 세미콜론을 붙여주지 않는 것을 확인할 수 있음.

    // Don't change the expression below!
    if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    }
}

fn main() {
    // You can optionally experiment here.
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
