use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::hash::Hash;
use std::io::Chain;
use std::time::Duration;

use crate::field::{self, get_top, ChainInfo, Field};
use crate::nazopuyo_info::NazopuyoInfo;
use crate::puyo_hash::PuyoHash;

// use std::sync::mpsc;
// use std::sync::mpsc::{Receiver, Sender};
// use std::thread::{self, ThreadId};

// use super::field::Field;
// type Hasher = std::hash::BuildHasherDefault<rustc_hash::FxHasher>;
// let mut hash: HashSet<PuyoHash, Hasher> = HashSet::default();
#[derive(Debug)]
pub struct Solver<F>
where
    F: Field + Clone + PartialEq + Eq + Hash + std::fmt::Display + Send + 'static,
{
    field: F, //Box<dyn Field>,
    info: NazopuyoInfo,
    hash: HashSet<PuyoHash>,
}

impl<F> Solver<F>
where
    F: Field + Clone + PartialEq + Eq + Hash + std::fmt::Display + Send + 'static,
{
    pub fn new(field: F, info: NazopuyoInfo) -> Self {
        Self {
            field,
            info,
            hash: HashSet::new(),
        }
    }

    pub fn from_url(url: &String) -> Self {
        let field = F::from_url(&url).unwrap();
        let info = NazopuyoInfo::from_url(&url).unwrap();
        let hash = HashSet::new();

        Self { field, info, hash }
    }

    pub fn solve(&mut self) -> Option<F> {
        let mut chain_info = ChainInfo::new();
        let is_strict = {
            let mut total = self.info.next.len * 2;
            for i in 0..13 {
                for j in 0..6 {
                    if self.field.get(i, j) >= 2 {
                        total += 1;
                    }
                }
            }
            total == (self.info.chain * 4) as usize
        };
        chain_info.set_is_strict(is_strict);

        match self.dfs(
            self.field.clone(),
            self.info.clone(),
            0,
            chain_info,
            PuyoHash::new(),
        ) {
            Some(field) => Some(field),
            None => None,
        }
    }

    fn dfs(
        &mut self,
        mut field: F,
        info: NazopuyoInfo,
        depth: u32,
        chain_info: ChainInfo,
        puyo_hash: PuyoHash,
    ) -> Option<F> {
        if depth == info.next.len as u32 {
            // let mut rnd = rand::thread_rng();
            // if rnd.gen::<u32>() % 1000 == 1 {
            //     println!("{}", field);
            // }
            let f2 = field.clone();
            // calclate sum of field.value, which is [[u8; 13]; 6];
            //let total = field.value.iter().map(|x| x.iter().sum::<u8>()).sum::<u8>();
            if field::chain(&mut field) == info.chain {
                // println!("{}", f2);
                //println!("{}", field);
                return Some(f2);
            } else {
                return None;
            }
        }

        let indicies = [2, 4, 3, 5, 1, 0];

        for j in indicies {
            for index in 0..2 {
                let y1 = match get_top(&field, j) {
                    Some(y) => match y {
                        0 => continue,
                        _ => y - 1,
                    },
                    None => 12,
                };
                field.set(y1, j, info.next.value[depth as usize][index]);
                let mut chain_info2 = ChainInfo::new();
                chain_info2.set_is_strict(chain_info.is_strict());
                chain_info2.set_changed(j);
                let puyo_hash2 = puyo_hash.insert(j, info.next.value[depth as usize][index]);

                for dx in 0..2 {
                    if j + dx >= 6 {
                        continue;
                    }
                    let y2 = match get_top(&field, j + dx) {
                        Some(y) => match y {
                            0 => {
                                // field.set(y1, j, 0);
                                continue;
                            }
                            _ => y - 1,
                        },
                        None => 12,
                    };
                    field.set(y2, j + dx, info.next.value[depth as usize][index ^ 1]);
                    let mut chain_info3 = chain_info2.clone();
                    chain_info3.set_changed(j + dx);
                    let puyo_hash3 =
                        puyo_hash2.insert(j + dx, info.next.value[depth as usize][index ^ 1]);

                    if !self.hash.contains(&puyo_hash3)
                        && (depth + 1 == info.next.len as u32
                            || (field.is_alive() && !field::has_chain(&field, chain_info3)))
                    {
                        self.hash.insert(puyo_hash3);
                        let mut new_chain_info = ChainInfo::new();
                        new_chain_info.set_is_strict(chain_info.is_strict());
                        new_chain_info.set_changed(j);
                        new_chain_info.set_changed(j + dx);
                        let res = self.dfs(
                            field.clone(),
                            info.clone(),
                            depth + 1,
                            new_chain_info,
                            puyo_hash3,
                        );
                        if res.is_some() {
                            return res;
                        }
                    }
                    field.set(y2, j + dx, 0);
                }
                field.set(y1, j, 0);
            }
        }

        None
    }

    /*
    pub fn solve_multi(&mut self) -> Option<F> {
        let (tx, rx) = mpsc::channel();

        let mut fields = vec![];
        let indicies = [2, 4, 3, 5, 1, 0];

        for j in indicies {
            for index in 0..2 {
                let y1 = match get_top(&self.field, j) {
                    Some(y) => match y {
                        0 => continue,
                        _ => y - 1,
                    },
                    None => 12,
                };
                self.field.set(y1, j, self.info.next.value[0][index]);
                for dx in 0..2 {
                    if j + dx >= 6 {
                        continue;
                    }
                    let y2 = match get_top(&self.field, j + dx) {
                        Some(y) => match y {
                            0 => {
                                // field.set(y1, j, 0);
                                continue;
                            }
                            _ => y - 1,
                        },
                        None => 12,
                    };
                    self.field
                        .set(y2, j + dx, self.info.next.value[0][index ^ 1]);
                    if !self.hash.contains(&self.field)
                        && self.field.is_alive()
                        && !field::has_chain(&self.field)
                    {
                        self.hash.insert(self.field.clone());
                        fields.push(self.field.clone());
                    }
                    self.field.set(y2, j + dx, 0);
                }
                self.field.set(y1, j, 0);
            }
        }

        println!("{} patterns!", fields.len());

        // for f in fields.iter() {
        //     println!("{}", f);
        // }
        let mut handles = Vec::new();
        let info2 = self.info.clone();
        for f in fields.into_iter() {
            let tx = tx.clone();
            let mut f2 = f.clone();
            let info3 = info2.clone();
            let mut hash2 = self.hash.clone();
            let handle = thread::spawn(move || {
                let res = parallel_dfs(&mut f2, info3, 1, hash2.borrow_mut());
                if res.is_some() {
                    println!("found!");
                    tx.send(res).ok();
                } else {
                    println!("No result for {:?}", thread::current().id());
                }
            });
            handles.push(handle);
            //thread::sleep(Duration::from_millis(1000));
        }

        // for handle in handles {
        //     handle.join().unwrap();
        // }
        drop(tx);

        rx.recv().ok().and_then(|opt| opt)
    }
    */
}
/*
fn parallel_dfs<F>(
    field: &mut F,
    info: NazopuyoInfo,
    depth: u32,
    hash: &mut HashSet<F>,
) -> Option<F>
where
    F: Field + Clone + PartialEq + Eq + Hash + std::fmt::Display + Send,
{
    if depth == info.next.len as u32 {
        let f2 = &mut field.clone();
        let f3 = field.clone();
        if field::chain(f2) == info.chain {
            return Some(f3);
        } else {
            return None;
        }
    }

    let indicies = [2, 4, 3, 5, 1, 0];

    for j in indicies {
        for index in 0..2 {
            let y1 = match get_top(field, j) {
                Some(y) => match y {
                    0 => continue,
                    _ => y - 1,
                },
                None => 12,
            };
            field.set(y1, j, info.next.value[depth as usize][index]);
            for dx in 0..2 {
                if j + dx >= 6 {
                    continue;
                }
                let y2 = match get_top(field, j + dx) {
                    Some(y) => match y {
                        0 => {
                            // field.set(y1, j, 0);
                            continue;
                        }
                        _ => y - 1,
                    },
                    None => 12,
                };
                field.set(y2, j + dx, info.next.value[depth as usize][index ^ 1]);

                if !hash.contains(&field)
                    && (depth + 1 == info.next.len as u32
                        || (field.is_alive() && !field::has_chain(field)))
                {
                    hash.insert(field.clone());
                    {
                        let res = parallel_dfs(field, info.clone(), depth + 1, hash.borrow_mut());
                        if res.is_some() {
                            return res;
                        }
                    }
                }
                field.set(y2, j + dx, 0);
            }
            field.set(y1, j, 0);
        }
    }
    None
}
*/
