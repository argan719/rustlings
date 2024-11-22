fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;
    // 'cat' 튜플을 구조 분해하여 'name'과 'age'라는 변수에 할당.
    let (name, age) = cat;

    println!("{name} is {age} years old");
}
