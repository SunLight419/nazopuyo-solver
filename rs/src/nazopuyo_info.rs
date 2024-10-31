use crate::naive_next_puyo::NaiveNextPuyo;

#[derive(Debug, Clone)]
pub struct NazopuyoInfo {
    pub chain: u32,
    pub next: NaiveNextPuyo,
}

impl NazopuyoInfo {
    pub fn from_url(url: &String) -> anyhow::Result<Self> {
        if !url.contains("?") {
            let message = format!("{} is not a valid url.", url);
            return anyhow::Result::Err(anyhow::anyhow!(message));
        }
        let param = url.split("__").collect::<Vec<_>>()[1]
            .chars()
            .collect::<Vec<_>>();
        if !(param[0] == 'u' && param[1] == '0') {
            let message = format!("{} is not a valid url. 条件がおかしそう", url);
            return anyhow::Result::Err(anyhow::anyhow!(message));
        }
        let chain = match param[2] {
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'a' => 10,
            'b' => 11,
            'c' => 12,
            'd' => 13,
            'e' => 14,
            'f' => 15,
            'g' => 16,
            'h' => 17,
            'i' => 18,
            'j' => 19,
            _ => todo!(),
        };

        let next = NaiveNextPuyo::from_url(&url).unwrap();
        
        Ok(Self { chain, next })
    }
}