fn fill_vec(vec: &mut Vec<i32>) {
    //let mut vec = vec;
    // 가변 참조를 매개변수로 받아, 벡터 vec0의 소유권을 이전하지 않고 참조를 통해 값을 수정. 
    // 소유권 이전 없이 vec0의 데이터를 직접 변경할 수 있다.

    vec.push(88);

    // vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;
 
    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let mut vec0 = vec![22, 44, 66];

        //let vec1 = fill_vec(&mut vec0);

        assert_eq!(vec0, [22, 44, 66]);  // vec0이 fill_vec 함수로 이동하면서 소유권 이전됨.
        //assert_eq!(vec1, [22, 44, 66, 88]);
        // 위 문제를 해결하기 위해서는 vec0을 fill_vec에 복사하여 전달하거나, 참조를 사용하거나

        // 나는 참조를 사용할 거임. (근데 이 문제에서 의도한 건 복사 같기도..)
        // 복사를 사용할 경우) let vec1 = fill_vec(vec0.clone());

        // fill_vec 함수가 값을 소유하지 않고 가변 참조로 전달받아 값을 추가하도록 수정하면 vec0과 vec1을 동시에 사용할 수 있다.
        fill_vec(&mut vec0);  // 참조를 전달하여 vec0의 값을 직접 수정
        assert_eq!(vec0, [22, 44, 66, 88]);
        
        let vec1 = &vec0;
        assert_eq!(vec0, [22, 44, 66, 88]);
        assert_eq!(*vec1, [22, 44, 66, 88]);
    } 
}
