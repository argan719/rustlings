#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use std::string;

    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();

        // TODO: Create your own order using the update syntax and template above!
        // let your_order =

        // let your_order {
        //     name: string,
        //     year: u8,
        //     made_by_phone: bool,
        //     made_by_mobile: bool,
        //     made_by_email: bool,
        //     item_number: u8,
        //     count: u8,
        // };


        let your_order = Order {
            name: String::from("Hacker in Rust"),
            count: 1,
            ..order_template  // 템플릿 필드를 복사하여 나머지 값 채우기
        };
        // 업데이트 구문 (..)
        // 탬풀릿을 기반으로 필요한 필드만 변경하고 나머지는 .. 구문을 사용하여 기존 템플릿의 값 복사

        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}
