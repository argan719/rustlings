fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    // input.trim()

    // 직접 로직 구현
    let bytes = input.as_bytes();  // 문자열을 바이트 배열로 변환
    let mut start = 0;
    let mut end = input.len();

    // 앞쪽 공백 제거
    while start < end && (bytes[start] as char).is_whitespace() {
        start += 1;
    }

    // 뒤쪽 공백 제거
    while end > start && (bytes[end-1] as char).is_whitespace() {
        end -= 1;
    }

    &input[start..end]
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    format!("{input} world!")
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    // input.replace("cars", "ballons")

    // 직접 로직 구현
    let mut result = String::new();
    let mut i = 0;

    while i < input.len() {
        if i + 4 <= input.len() && &input[i..i + 4] == "cars" {
            result.push_str("ballons");
            i += 4;  // "cars" 길이만큼 이동
        } else {
            result.push(input.chars().nth(i).unwrap());
            i += 1;
        }
    }
    result
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
