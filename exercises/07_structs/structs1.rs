struct ColorRegularStruct {  // 이름이 있는 필드로 구성된 구조체
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?
    red: u8,
    green: u8,
    blue: u8,
}

// struct ColorTupleStruct(/* TODO: Add the fields that the test `tuple_structs` expects */);
struct ColorTupleStruct(u8, u8, u8);  // 필드 이름이 없는 튜플 구조체.

#[derive(Debug)]  // Debug 트레이트
struct UnitStruct;  // 필드가 없는 구조체. 데이터 없이 이름만 존재.

fn main() {
    // You can optionally experiment here.
}
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        // let green =
        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        // let green =
        let green = ColorTupleStruct(0, 255, 0);
        // ColorTupleStruct을 통해 green 색상 생성.
        // 각 값에 대한 인덱스를 사용하여 접근.

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        // let unit_struct =
        let unit_struct = UnitStruct;

        let message = format!("{unit_struct:?}s are fun!");  // format! 매크로를 사용해 디버그 출력으로 문자열 생성

        assert_eq!(message, "UnitStructs are fun!");
    }
}
