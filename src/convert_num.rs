pub fn seporate_num(num: f32) -> (f32, u8) {
    let dig = num % 10.0;

    let rest = (num - dig) / 10.0;

    (rest, dig.round() as u8)
}

pub fn convert_num(num: f32) -> (u8, u8, u8) {
    let (num, c) = seporate_num(num);
    let (num, b) = seporate_num(num);
    let (_, a) = seporate_num(num);

    (a, b, c)
}
