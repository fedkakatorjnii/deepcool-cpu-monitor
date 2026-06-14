pub fn seporate_num(num: f32) -> (f32, u8) {
    let dig = num % 10.0;
    let rest = (num - dig) / 10.0;

    (rest, dig.round() as u8)
}

pub fn convert_num(num: f32) -> (u8, u8, u8) {
    let num = num.round();
    let (num, c) = seporate_num(num);
    let (num, b) = seporate_num(num);
    let (_, a) = seporate_num(num);

    (a, b, c)
}

#[cfg(test)]
mod tests {
    use super::*; // Bring the outer code into scope

    #[test]
    fn test_convert_num_1() {
        let num = 50.00;
        let actual = convert_num(num);
        let expected = (0, 5, 0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_convert_num_2() {
        let num = 50.49999;
        let actual = convert_num(num);
        let expected = (0, 5, 0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_convert_num_3() {
        let num = 50.5000;
        let actual = convert_num(num);
        let expected = (0, 5, 1);
        assert_eq!(actual, expected);
    }
}
