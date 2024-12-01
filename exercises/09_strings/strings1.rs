// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    "blue".to_string()
}

// 기존 "blue"는 문자열 리터럴로(&str),정적 메모리에 저장된다. 
// 이를 String으로 바꿔줘야 한다.
// 이와 달리, String은 힙(heap)에 저장되는 소유권을 가진 타입.

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
    