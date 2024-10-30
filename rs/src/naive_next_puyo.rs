use std::fmt;
use colored::*;
use crate::constants;

#[derive(Debug, Clone)]
pub struct NaiveNextPuyo {
    pub value: [[u8; 2]; 10],
    pub len: usize,
}

impl NaiveNextPuyo {
    pub fn new() -> Self {
        NaiveNextPuyo {
            value: [[0u8; 2]; 10],
            len: 0,
        }
    }

    pub fn from_vec(v: Vec<[u8; 2]>) -> Self {
        NaiveNextPuyo {
            len: v.len(),
            value: v.try_into().unwrap(),
        }
    }

    pub fn from_url(url: &String) -> anyhow::Result<NaiveNextPuyo> {
        if !url.contains("?") {
            let message = format!("{} is not a valid url.", url);
            return anyhow::Result::Err(anyhow::anyhow!(message));
        }
        let param = url.split("_").collect::<Vec<_>>()[1]
            .chars()
            .collect::<Vec<_>>();
        let n = param.len();
        if n % 2 != 0 {
            let message = format!("{:?} is not a valid parameter.", param);
            return anyhow::Result::Err(anyhow::anyhow!(message));
        }

        // let e = constants::EMPTY;
        // let o = constants::OJAMA;
        let r = constants::RED;
        let b = constants::BLUE;
        let g = constants::GREEN;
        let y = constants::YELLOW;
        let p = constants::PURPLE;

        let mut value = [[0, 0]; 10];
        let mut res = NaiveNextPuyo::new();
        res.len = n / 2;
        for i in 0..(n / 2) {
            let c = param[2 * i];
            let (left, right) = match c {
                '0' => (r, r),
                '2' => (g, g),
                '4' => (b, b),
                '6' => (y, y),
                '8' => (p, p),
                'c' => (r, g),
                'e' => (g, g),
                'g' => (b, g),
                'i' => (y, g),
                'k' => (p, g),
                'o' => (r, b),
                'q' => (g, b),
                's' => (b, b),
                'u' => (y, b),
                'w' => (p, b),
                'A' => (r, y),
                'C' => (g, y),
                'E' => (b, y),
                'G' => (y, y),
                'I' => (p, y),
                'M' => (r, p),
                'O' => (g, p),
                'Q' => (b, p),
                'S' => (y, p),
                'U' => (p, p),
                _ => todo!(),
            };
            value[i] = [left, right];
        }
        res.value = value;
        Ok(res)
    }
}

impl fmt::Display for NaiveNextPuyo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for j in 0..2 {
            for i in 0..self.len {
                let s = match self.value[i][j] {
                    1 => "@".color("white"),
                    2 => "2".color("red"),
                    3 => "3".color("blue"),
                    4 => "4".color("green"),
                    5 => "5".color("yellow"),
                    6 => "6".color("magenta"),
                    _ => "-".color("white"),
                };
                write!(f, "{:2} ", s)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}