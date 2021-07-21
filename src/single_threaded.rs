fn prim_check_single_number(smaller_prims: &Vec<u64>, number: &u64) -> bool {
    for p in smaller_prims {
        if (number % p) == 0 {
            return false;
        }
        else if (p * p) > *number {
            return true;
        }
    }
    return true;
}

pub fn prim_count(end: &u64) -> Vec<u64> {
    let mut smaller_prims: Vec<u64> = Vec::new();
    for num in 2..*end {
        if prim_check_single_number(&smaller_prims, &num) {
            // println!("New prime: {}", &num);
            smaller_prims.push(num);
        }
    }
    smaller_prims
}