pub fn nth(n: u32) -> u32 {
    let mut index : u32 = 0;
    let mut cpt: u32 = n;
    while cpt > 0 {
        if is_prime(index) {
            index = index + 1;
        }
        cpt = cpt - 1;
    }
    index
}

pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    return n % n == 0 && n % 1 == 0;
}