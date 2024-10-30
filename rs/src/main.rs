mod constants;
mod field;
mod field_naive_bit;
mod naive_next_puyo;

use field::Field;
use field_naive_bit::FieldNaiveBit;
use naive_next_puyo::NaiveNextPuyo;

fn main() {
    // field::kenny_bench::<FieldNaiveBit>();
    let url =
        r"https://ishikawapuyo.net/simu/pn.html?5F85r81qg1aw3sw2p82k81Ao1bo_o1c1u1k1A1o1__u0c"
            .to_string();
    let field = FieldNaiveBit::from_url(&url).unwrap();
    println!("{field}");

    let np = NaiveNextPuyo::from_url(&url).unwrap();
    println!("{}", np);

}
