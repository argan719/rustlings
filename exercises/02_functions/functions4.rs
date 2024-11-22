// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
// Don't worry about the function bodies themselves, we are only interested in
// the signatures for now.

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: Fix the function signature.
// sale_price 함수의 반환 타입을 명시하지 않았기 때문에 컴파일 에러 발생.
fn sale_price(price: i64) -> i64 {
    if is_even(price) {
        price - 10  // 반환값이 i64로 존재하므로 반환 타입을 명시해줘야 함.
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}
