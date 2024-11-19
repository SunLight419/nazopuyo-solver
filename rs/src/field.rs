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
    /// 一番上にあるぷよのインデックス
    fn get_top(&self, x: usize) -> Option<usize> {
        for i in 0..13 {
            if self.get(i, x) != 0 {
                return Some(i);
            }
        }
        None
    }
}

pub fn from_url<F: Field>(url: &String) -> Result<F, Box<dyn std::error::Error>> {
    if !url.contains("?") {
        let message = format!("{} is an invalid url.", url);
        return Err(message.into());
    }
    let query = url.split("?").skip(1).next().unwrap();
    let field_url: Vec<_> = query.chars().take_while(|c| *c != '_').collect();

    let mut field = F::new();

    let mut y = 12;
    let mut x = 4;
    for c in field_url.into_iter().rev() {
        // TODO: 数字を直接参照ではなく変数にかませるようにする
        // というか対応の const を作成する
        let empty = 0;
        let ojama = 1;
        let red = 2;
        let blue = 3;
        let green = 4;
        let yellow = 5;
        let purple = 6;

        let (left, right) = match c {
            '0' => (empty, empty),
            '1' => (empty, red),
            '2' => (empty, green),
            '3' => (empty, blue),
            '4' => (empty, yellow),
            '5' => (empty, purple),
            '6' => (empty, ojama),
            '7' => todo!(),
            '8' => (red, empty),
            '9' => (red, red),
            'a' => (red, green),
            'b' => (red, blue),
            'c' => (red, yellow),
            'd' => (red, purple),
            'e' => (red, ojama),
            'f' => todo!(),
            'g' => (green, empty),
            'h' => (green, red),
            'i' => (green, green),
            'j' => (green, blue),
            'k' => (green, yellow),
            'l' => (green, purple),
            'm' => (green, ojama),
            'n' => todo!(),
            'o' => (blue, empty),
            'p' => (blue, red),
            'q' => (blue, green),
            'r' => (blue, blue),
            's' => (blue, yellow),
            't' => (blue, purple),
            'u' => (blue, ojama),
            'v' => todo!(),
            'w' => (yellow, empty),
            'x' => (yellow, red),
            'y' => (yellow, green),
            'z' => (yellow, blue),
            'A' => (yellow, yellow),
            'B' => (yellow, purple),
            'C' => (yellow, ojama),
            'D' => todo!(),
            'E' => (purple, empty),
            'F' => (purple, red),
            'G' => (purple, green),
            'H' => (purple, blue),
            'I' => (purple, yellow),
            'J' => (purple, purple),
            'K' => (purple, ojama),
            'L' => todo!(),
            'M' => (ojama, empty),
            'N' => (ojama, red),
            'O' => (ojama, green),
            'P' => (ojama, blue),
            'Q' => (ojama, yellow),
            'R' => (ojama, purple),
            'S' => (ojama, ojama),
            'T' => todo!(),
            _ => todo!(),
        };
        field.set(y, x, left);
        field.set(y, x + 1, right);

        if x >= 2 {
            x -= 2;
        } else {
            x = 4;
            y -= 1;
        }
    }
    Ok(field)
}

/// 一番上にあるぷよのインデックス
/// None => 列が空
/// ✟伝家の宝刀 人力二分探索✟
/// TODO ぷよを置ける位置に変更した方がよい？
pub fn get_top<F: Field>(field: &F, x: usize) -> Option<usize> {
    if field.get(12, x) == 0 {
        return None;
    }
    if field.get(6, x) != 0 {
        if field.get(3, x) != 0 {
            if field.get(1, x) != 0 {
                if field.get(0, x) != 0 {
                    return Some(0);
                } else {
                    return Some(1);
                }
            } else {
                if field.get(2, x) != 0 {
                    return Some(2);
                } else {
                    return Some(3);
                }
            }
        } else {
            if field.get(4, x) != 0 {
                return Some(4);
            } else if field.get(5, x) != 0 {
                return Some(5);
            } else {
                return Some(6);
            }
        }
    }

    if field.get(9, x) != 0 {
        if field.get(7, x) != 0 {
            return Some(7);
        } else if field.get(8, x) != 0 {
            return Some(8);
        } else {
            return Some(9);
        }
    }

    if field.get(10, x) != 0 {
        return Some(10);
    }

    if field.get(11, x) != 0 {
        return Some(11);
    }

    Some(12)
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
pub fn kenny_bench<T>(comment: String)
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
        let num = field::chain(f);
        assert_eq!(num, 19);
    }

    let t_end = Instant::now();
    let ell = t_end - t_start;
    println!("{:?} {:?}", ell, ell / count as u32);

    use std::io::Write;
    let filepath = r".\kenny_bench_result.txt";
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(filepath);

    match file {
        Ok(mut f) => {
            let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            writeln!(f, "[{}] {}", now, comment);
            writeln!(f, "Type of {}", std::any::type_name::<T>());
            writeln!(f, "{:?} {:?}", ell, ell / count as u32);
        }
        _ => {
            println!("Failed to write kenny bench result in {filepath}");
        }
    }
}
