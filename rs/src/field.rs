use std::{collections::VecDeque, error::Error};

#[derive(Clone, Copy, Debug)]
pub struct ChainInfo {
    info: usize,
}

impl ChainInfo {
    pub fn new() -> Self {
        Self { info: 0 }
    }
    pub fn is_strict(&self) -> bool {
        self.info & 6 > 0
    }
    pub fn set_is_strict(&mut self, is_strict: bool) {
        if is_strict {
            self.info |= 1;
        } else {
            self.info &= !1
        }
    }
    pub fn set_changed(&mut self, col: usize) {
        self.info |= 1 << col;
    }
    pub fn is_changed(&self, col: usize) -> bool {
        (self.info & (1 << col)) > 0
    }
}

#[cfg(test)]
mod tests {
    use super::ChainInfo;
    #[test]
    fn is_changed_test() {
        let mut info = ChainInfo::new();
        info.set_changed(1);
        assert_eq!(info.is_changed(1), true);
    }
}

pub trait Field {
    fn new() -> Self
    where
        Self: Sized;
    fn from_u8(value: [[u8; 13]; 6]) -> Self;
    fn from_char(value: [[char; 13]; 6]) -> Self;
    fn from_url(url: &String) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    // fn from(v: [[u32; 13]; 6]) -> Self;
    fn set(&mut self, y: usize, x: usize, v: u8);
    fn get(&self, y: usize, x: usize) -> u8;
    // todo? field::fall() に切り出すべき？
    fn fall(&mut self);
    //fn chain(&mut self) -> u32;
    fn is_empty(&self) -> bool;
    fn is_alive(&self) -> bool;
    // fn top(&self, x: usize) -> usize;
}

pub fn chain<F: Field>(field: &mut F) -> u32
where
    F: Clone,
{
    let mut chain_count = 0u32;
    let dy = [0i32, 1, 0, -1];
    let dx = [1i32, 0, -1, 0];
    let mut chain_info = ChainInfo::new();
    for j in 0..6 {
        chain_info.set_changed(j);
    }

    loop {
        let mut chained = false;
        let mut vis = [[false; 6]; 13];

        let mut next_field = field.clone();
        let mut new_chain_info = ChainInfo::new();
        new_chain_info.set_is_strict(chain_info.is_strict());

        for i in 1..13 {
            for j in 0..6 {
                if !chain_info.is_changed(j) {
                    continue;
                }

                if vis[i][j] {
                    continue;
                }
                vis[i][j] = true;
                let color = next_field.get(i, j);
                if color <= 1 {
                    continue;
                }
                let mut que = VecDeque::new();
                let mut connected = VecDeque::new();

                que.push_back((i, j));
                connected.push_back((i, j));

                while !que.is_empty() {
                    let (y, x) = que.pop_front().unwrap();
                    for k in 0..4 {
                        let ny = y as i32 + dy[k];
                        let nx = x as i32 + dx[k];
                        if nx < 0
                            || ny < 0
                            || 6 <= nx
                            || 13 <= ny
                            || vis[ny as usize][nx as usize]
                            || ny == 0
                        {
                            continue;
                        }
                        let new_color = next_field.get(ny as usize, nx as usize);
                        if new_color != color
                        /*|| new_color <= 1*/
                        {
                            continue;
                        }
                        vis[ny as usize][nx as usize] = true;
                        que.push_back((ny as usize, nx as usize));
                        connected.push_back((ny as usize, nx as usize));
                    }
                }

                if connected.len() >= 4 {
                    chained = true;
                    // if new_chain_info.is_strict() && connected.len() >= 5 {
                    //     ;
                    // }
                    for (y, x) in connected {
                        new_chain_info.set_changed(x);
                        next_field.set(y, x, 0);

                        // おじゃまぷよ
                        for k in 0..4 {
                            let ny = y as i32 + dy[k];
                            let nx = x as i32 + dx[k];
                            if nx < 0
                                || ny < 0
                                || 6 <= nx
                                || 13 <= ny
                                || ny == 0
                                || next_field.get(ny as usize, nx as usize) != 1
                            {
                                continue;
                            }
                            next_field.set(ny as usize, nx as usize, 0);
                        }
                    }
                }
            }
        }

        if !chained {
            break;
        }

        chain_info = new_chain_info;

        chain_count += 1;
        *field = next_field;
        field.fall();
    }

    chain_count
}

pub fn has_chain<F: Field>(field: &F, chain_info: ChainInfo) -> bool {
    let dy = [0i32, 1, 0, -1];
    let dx = [1i32, 0, -1, 0];

    let mut vis = [[false; 6]; 13];

    for i in 1..13 {
        for j in 0..6 {
            if !chain_info.is_changed(j) {
                continue;
            }
            if vis[i][j] {
                continue;
            }
            vis[i][j] = true;
            let color = field.get(i, j);
            if color <= 1 {
                continue;
            }
            let mut que = VecDeque::new();
            let mut connected = VecDeque::new();

            que.push_back((i, j));
            connected.push_back((i, j));

            while !que.is_empty() {
                let (y, x) = que.pop_front().unwrap();
                for k in 0..4 {
                    let ny = y as i32 + dy[k];
                    let nx = x as i32 + dx[k];
                    if nx < 0
                        || ny < 0
                        || 6 <= nx
                        || 13 <= ny
                        || vis[ny as usize][nx as usize]
                        || ny == 0
                    {
                        continue;
                    }
                    let new_color = field.get(ny as usize, nx as usize);
                    if new_color != color
                    /*|| new_color == 0 || new_color == 1*/
                    {
                        continue;
                    }
                    vis[ny as usize][nx as usize] = true;
                    que.push_back((ny as usize, nx as usize));
                    connected.push_back((ny as usize, nx as usize));
                }
            }

            if connected.len() >= 4 {
                return true;
            }
        }
    }

    false
}

#[allow(unused)]
pub fn kenny_bench<T>()
where
    T: Field + Clone,
{
    use crate::field;
    use std::time::Instant;
    let count = 1_000_000;
    let mut fields = Vec::with_capacity(count);
    for _ in 0..count {
        let kenny = [
            [0u8, 5, 6, 5, 6, 3, 6, 6, 6, 5, 6, 6, 6],
            [0u8, 4, 4, 4, 3, 4, 3, 3, 5, 3, 5, 5, 5],
            [5u8, 4, 5, 5, 4, 5, 4, 4, 5, 4, 3, 3, 3],
            [6u8, 5, 6, 6, 5, 6, 5, 5, 4, 6, 4, 4, 4],
            [3u8, 6, 3, 3, 6, 3, 6, 6, 5, 4, 6, 6, 6],
            [3u8, 4, 3, 4, 4, 4, 3, 3, 4, 4, 5, 5, 5],
        ];
        let field = T::from_u8(kenny);
        fields.push(field);
    }
    println!(
        "{} loop kenny test: type of {}",
        count,
        std::any::type_name::<T>()
    );
    let t_start = Instant::now();

    for f in fields.iter_mut() {
        field::chain(f);
    }

    let t_end = Instant::now();
    let ell = t_end - t_start;
    println!("{:?} {:?}", ell, ell / count as u32);
}
