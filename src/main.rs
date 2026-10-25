use num_bigint::BigUint;

fn main() {
    let mut a: BigUint = BigUint::from(0u8);
    let mut b: BigUint = BigUint::from(1u8);
    let mut r: BigUint;

    loop {
        r = a + b.clone();
        a = b.clone();
        b = r.clone();

        println!("{}\n", r)
    }
}
