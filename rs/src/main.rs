mod constants;
mod field;
mod field_bitboard;
mod field_naive_bit;
mod field_underwater;
mod naive_next_puyo;
mod nazopuyo_info;
mod nazopuyo_solver;
mod nazopuyo_solver_underwater;
mod puyo_hash;

use field::Field;
use field_underwater::FieldUW;
use field_naive_bit::FieldNaiveBit;
use naive_next_puyo::NaiveNextPuyo;
use nazopuyo_info::NazopuyoInfo;
use nazopuyo_solver::Solver;

fn main() {
    // field::kenny_bench::<FieldNaiveBit>("コメント機能を追加 Windows".to_string());
    // let url =
    // //r"https://ishikawapuyo.net/simu/ps.html?20010860M20M1iE_O1c1810121O1__u05".to_string();
    //    r"https://ishikawapuyo.net/simu/pn.html?5F85r81qg1aw3sw2p82k81Ao1bo_o1c1u1k1A1o1__u0c"
    //        .to_string();
    //     // https://ishikawapuyo.net/simu/pn.html?3o838AQwxhwaa8bj8xpwAcw_e1c1A1A1C1o1u1__u0c
    // let mut field = FieldUW::from_url(&url).unwrap().set_water_height(8);

    let mut field = FieldUW::new().set_water_height(8);
    field.set(6, 1, 1);
    field.set(7, 1, 3);
    field.set(8, 1, 2);
    field.set(9, 1, 2);
    field.set(10, 1, 1);
    field.set(11, 1, 2);

    field.set(6, 2, 3);
    field.set(7, 2, 1);
    field.set(8, 2, 2);
    field.set(9, 2, 1);
    field.set(10, 2, 3);
    field.set(11, 2, 4);
    field.set(12, 2, 4);

    field.set(6, 3, 2);
    field.set(7, 3, 3);
    field.set(8, 3, 3);
    field.set(9, 3, 2);
    field.set(10, 3, 2);

    println!("{field}");
    // field.set(0, 0, 2);
    // field.set(0, 1, 2);
    // field.fall();
    // println!("{field}");

    let mut info = NazopuyoInfo {
        chain: 5,
        next: NaiveNextPuyo::new(),
    };
    info.next.len = 3;
    info.next.value[0] = [3, 2];
    info.next.value[1] = [3, 4];
    info.next.value[2] = [3, 4];
    let mut solver = nazopuyo_solver_underwater::Solver::new(field, info);
    
    let res = solver.solve().unwrap();

    println!("{res}");


    return;

    // let np = NaiveNextPuyo::from_url(&url).unwrap();
    // println!("{}", np);

    // let info = NazopuyoInfo::from_url(&url).unwrap();
    // println!("{:?}", info);

    // let start = std::time::Instant::now();
    // let mut solver = Solver::<FieldNaiveBit>::from_url(&url);
    // let res = solver.solve().unwrap();
    // let end = std::time::Instant::now();
    // println!("{:?}", end - start);
    // println!("{}", res);
}
