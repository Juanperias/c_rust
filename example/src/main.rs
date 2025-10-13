use c_rust::{c_rust, parse_c, c_ty, gen_body};

c_rust! {
    int sum(int a, int b) {
        return a + b;
    }
}

fn main() {
    let r = unsafe { sum(2, 3) };

    println!("{}", r);
}
