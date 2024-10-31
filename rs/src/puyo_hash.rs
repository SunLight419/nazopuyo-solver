use std::u64;

/// ぷよがどこに置かれたかを記憶します
#[derive(Clone, Copy)]
struct PuyoHash {
    value: u64,
}

impl PuyoHash {
    fn insert(self, mut col: usize, color: u8) -> Self {

        let mut mask = 0b111;
        let mut index = 0;
        let width = 3;
        while col > 0 {
            if (self.value & mask) == 0 {
                col -= 1;
            }
            mask <<= width;
            index += width;
        }

        let mask = (1 << index) - 1;
        let value = (self.value & mask) | ((color as u64) << index) | ((self.value & !mask) << width);
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::PuyoHash;

    #[test]
    fn it_works() {
        let ph = PuyoHash { value: 0 };
        let sut = ph.insert(1, 2);
        assert_eq!(sut.value, 0b_010_000);

        let sut = ph.insert(0, 7);
        assert_eq!(sut.value, 0b_111);

        let ph = PuyoHash {
            value: 0b_111_111_000_111,
        };
        let sut = ph.insert(1, 2);
        assert_eq!(sut.value, 0b_111_111_010_000_111);
    }
}
