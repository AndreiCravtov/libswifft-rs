use crate::reference::constant::{INPUT_BLOCK_SIZE, INPUT_SIZE, M, N, OMEGA_ORDER, OMEGA_POWERS, P, MULTIPLIER_COEFFICIENTS};
use crate::reference::polynomial::Polynomial;

pub type Input = [u8; INPUT_BLOCK_SIZE];
pub type Output = [u16; N];
pub fn nth_bit(arr: &[u8], n: usize) -> u8 {
    // Step 1: Determine the byte index
    let byte_index = n / 8;

    // Step 2: Determine the bit position within the byte
    let bit_position = n % 8;

    // Step 3: Extract the bit
    // Shift the target byte to the right so that the bit of interest is the LSB,
    // then bitwise AND with 1 to isolate it.
    let bit = (arr[byte_index] >> bit_position) & 1;

    bit
}


pub fn hash() {
    let ss = [0; N];

    let a = Polynomial::new([0; 64]);

    ss.map(|v| {v});

}


pub fn compute(input: &Input) -> Output {
    // map each input vector to a fourier-transformed vector
    // let mut fourier_output: [u16; INPUT_SIZE] = [0; INPUT_SIZE];
    // for j in 0..M { // each input vector
    //     for i in 0..N { // each element
    //         for k in 0..N {
    //             let x_index = j*N + k;
    //             let x_k = nth_bit(input, x_index) as u16;
    //             let omega_power = OMEGA_POWERS[((2*i + 1) * k) % OMEGA_ORDER];
    //             let product = (x_k * omega_power).rem_euclid(P);
    //             fourier_output[j*N + i] = (fourier_output[j*N + i] + product).rem_euclid(P);
    //         }
    //     }
    // }

    // fourier_output.iter().for_each(|value| { print!("{value} ") });
    // println!("\n\n");

    // computing 64 distinct linear combinations (modulo 257)
    // across the ith entries of the 16 y_j vectors
    let mut output: Output = [0; N];
    // for i in 0..N {
    //     for j in 0..M {
    //         let linear_combination = (fourier_output[i + j*N] * MULTIPLIER_COEFFICIENTS[i + j*N]).rem_euclid(P);
    //         // print!("{} ", linear_combination);
    //         output[i] = (output[i] + linear_combination).rem_euclid(P)
    //     }
    //     // println!();
    // }
    output
}

pub fn compute_multiple<const NUM_BLOCKS: usize>(input: &[Input; NUM_BLOCKS],
                                                 output: &mut [Output; NUM_BLOCKS]) {
}