use crate::constants::{Case, LIGHT, Mode};
use crate::convert_num::convert_num;

pub struct Data {
    pub light: u8,
    pub case: Case,
    pub mode: Mode,
    pub data: f32,
}

impl Data {
    pub fn new(case: Case, data: f32) -> Self {
        let light = LIGHT;
        let mode = Mode::Solid;

        Self {
            light,
            case,
            mode,
            data,
        }
    }

    pub fn convert(&self) -> [u8; 64] {
        let (a, b, c) = convert_num(self.data);
        let percent = (self.data / 10.0).round() as u8;

        let mut data = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];
        data[0] = self.light;
        data[1] = self.case as u8;
        data[2] = percent;
        data[3] = a;
        data[4] = b;
        data[5] = c;
        data[6] = self.mode as u8;

        data
    }
}
