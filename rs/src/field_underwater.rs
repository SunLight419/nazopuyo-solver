use crate::field::{self, Field};
use colored::*;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct FieldUW {
    value: [u64; 6],
    water_height: usize,
}

impl fmt::Display for FieldUW {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..13 {
            for j in 0..6 {
                let s = match self.get(i, j) {
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

impl Field for FieldUW {
    fn new() -> Self {
        Self {
            value: [0; 6],
            water_height: 0,
        }
    }

    fn from_u8(value: [[u8; 13]; 6]) -> Self {
        todo!()
    }

    fn from_char(value: [[char; 13]; 6]) -> Self {
        todo!()
    }

    fn from_url(url: &String) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        field::from_url::<Self>(&url)
    }

    #[inline]
    fn set(&mut self, y: usize, x: usize, v: u8) {
        let mask = 7u64 << (y * 3);
        self.value[x] = self.value[x] & !mask | ((v as u64) << (y * 3));
    }

    #[inline]
    fn get(&self, y: usize, x: usize) -> u8 {
        let mask = 7u64 << (y * 3);
        ((self.value[x] & mask) >> (y * 3)) as u8
    }

    fn fall(&mut self) {
        for j in 0..6 {
            if self.value[j] == 0 {
                continue;
            }
            let mut res = 0u64;
            let mut mask = 0b111u64;
            for _ in 0..13 {
                if self.value[j] & mask == 0 {
                    res <<= 3;
                } else {
                    res |= self.value[j] & mask;
                }
                mask <<= 3;
            }
            self.value[j] = res;
        }

        self.float();
    }

    #[inline]
    fn is_alive(&self) -> bool {
        (self.value[2] & 0b111000) == 0
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.value.iter().all(|x| *x == 0)
    }
}

impl FieldUW {
    pub fn set_water_height(mut self, height: usize) -> Self {
        self.water_height = height;
        self
    }

    pub fn float(&mut self) {
        for j in 0..6 {
            let count = {
                let mut res = 0;
                for i in 0..13 {
                    if self.get(i, j) != 0 {
                        res += 1;
                    }
                }
                res
            };
            if count == 0 || count >= self.water_height {
                continue;
            }
            let dif = self.water_height - count;
            self.value[j] >>= 3 * dif;
        }
    }
}