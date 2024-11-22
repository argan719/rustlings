fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
    // 배열 선언
    let a = [0; 100];  // 길이가 100인 배열, 모든 요소는 0으로 초기화

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
