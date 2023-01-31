use rand::Rng;

fn modular_pow(mut base: u128, mut exponent: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus
        }
        exponent >>= 1;
        base = (base * base) % modulus;
    }

    result
}

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g as u128, a as u128, p as u128) as u64
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub as u128, a as u128, p as u128) as u64
}
