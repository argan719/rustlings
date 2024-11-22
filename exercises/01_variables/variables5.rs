fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    // 새로운 변수 선언, Shadowing 활용
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}
