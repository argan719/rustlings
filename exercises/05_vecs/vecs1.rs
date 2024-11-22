fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.
    // let v = ???;

    // 이런식으로 벡터 타입을 명시할 수 있지만 러스트에서는 값의 타입을 유추해주므로
    // 굳이 타입 명시를 할 필요가 없다. 대신 편의를 위해 제공되는 vec! 매크로를 사용하면 된다!
    let v: Vec<i32> = Vec::new();
    
    // 매크로 사용하여 1,2,3을 저장하고 있는 Vec<i32> 생성
    let v = vec![1, 2, 3]; 

    // a와 같은 원소를 가진 벡터 v 생성
    let v = vec![10, 20, 30, 40];

    (a, v)
}

fn main() {
    // You can optionally experiment here.
    array_and_vec();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
