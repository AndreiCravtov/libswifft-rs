use libswifft_sys::reference::constant::{INPUT_BLOCK_SIZE, N};
use libswifft_sys::reference::polynomial::Polynomial;

pub mod sys {
    pub use libswifft_sys::buffer;
    pub use libswifft_sys::hash;
    pub use libswifft_sys::arithmetic;
    pub use libswifft_sys::reference;
}

fn main() {
    let a = Polynomial::new([
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
    ]);
    let b = a;

    println!("{}", a * b);
}