use crate::field::{self, Field};
use colored::*;
use std::fmt;
use core::arch::x86_64::*;

// #[cfg(target_feature = "avx")]
#[derive(Debug, Clone)]
pub struct FieldBitboard {
    value: __m256i,
}

// #[cfg(not(target_feature = "avx"))]
// #[derive(Debug, Clone)]
// pub struct FieldBitboard {
//     value: [u128; 2],
// }

impl fmt::Display for FieldBitboard {
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

impl Field for FieldBitboard {
    fn new() -> Self {
        Self { value: unsafe {
            _mm256_setzero_si256()
        } }
    }

    fn from_u8(value: [[u8; 13]; 6]) -> Self {
        todo!()
    }

    fn from_char(value: [[char; 13]; 6]) -> Self {
        todo!()
    }

    fn from_url(url: &String) -> Result<Self, Box<dyn std::error::Error>>
        where
            Self: Sized {
        field::from_url::<Self>(&url)
    }

    fn set(&mut self, y: usize, x: usize, v: u8) {
        todo!()
    }

    fn get(&self, y: usize, x: usize) -> u8 {
        todo!()
    }

    fn fall(&mut self) {
        todo!()
    }

    #[inline]
    fn is_alive(&self) -> bool {
        self.get(1, 2) == 0
    }

    #[inline]
    fn is_empty(&self) -> bool {
        let left = self.left();
        if (unsafe{ _mm_extract_epi64(left, 0) as u64}).count_ones() != 0 {
            return false;
        }

        if (unsafe{ _mm_extract_epi64(left, 1) as u64}).count_ones() != 0 {
            return false;
        }

        let right = self.right();
        if (unsafe{ _mm_extract_epi64(right, 0) as u64}).count_ones() != 0 {
            return false;
        }
        (unsafe{ _mm_extract_epi64(right, 1) as u64}).count_ones() == 0
    }
}


impl FieldBitboard {
    fn left(&self) -> __m128i {
        unsafe { _mm256_extracti128_si256(self.value, 0) }
    }

    fn right(&self) -> __m128i {
        unsafe { _mm256_extracti128_si256(self.value, 1) }
    }
}