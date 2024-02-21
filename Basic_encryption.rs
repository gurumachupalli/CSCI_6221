extern crate rand;
use rand::Rng;

// Parameters
const N: usize = 512;
const Q: i32 = 12289;

// Key Generation
fn keygen() -> (Vec<i32>, Vec<i32>) {
    let mut rng = rand::thread_rng();
    let mut s: Vec<i32> = Vec::with_capacity(N);
    let mut e: Vec<i32> = Vec::with_capacity(N);
    for _ in 0..N {
        s.push(rng.gen_range(0..Q));
        e.push(rng.gen_range(0..Q));
    }
    (s, e)
}

// Encryption
fn encrypt(s: &Vec<i32>, m: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut a: Vec<i32> = Vec::with_capacity(N);
    let mut e: Vec<i32> = Vec::with_capacity(N);
    for _ in 0..N {
        a.push(rng.gen_range(0..Q));
        e.push(rng.gen_range(0..Q));
    }
    let mut b: Vec<i32> = Vec::with_capacity(N);
    for i in 0..N {
        b.push((a[i] * s[i] + e[i]) % Q);
    }
    b.push(m);
    b
}

// Decryption
fn decrypt(s: &Vec<i32>, b: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in 0..N {
        sum += s[i] * b[i];
    }
    sum % Q
}

fn main() {
    let (s, _) = keygen();
    let m = 42; // Message to be encrypted
    let b = encrypt(&s, m);
    let decrypted_m = decrypt(&s, &b);
    println!("Decrypted message: {}", decrypted_m);
}
