use std::thread;
use std::sync::mpsc;

static NTHREADS: u64 = 16;

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

fn prim_get_numbers(smaller_prims: &Vec<u64>, start: u64, end: u64) -> Vec<u64>{
    let mut ret = Vec::new();

    for x in start..end {
        if prim_check_single_number(smaller_prims, &x) {
            ret.push(x);
        }
    }

    ret
}

fn divide_among_threads(smaller_prims: &mut Vec<u64>, start: u64, end: u64) {
    let slice_size : u64 = end-start;
    let slice_size_per_thread : u64 = slice_size / NTHREADS;
    let (tx,rx) : (mpsc::Sender<(u64,Vec<u64>)>, mpsc::Receiver<(u64,Vec<u64>)>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        let thread_tx = tx.clone();
        let thread_start = start + slice_size_per_thread * id;
        let thread_smaller_prims = smaller_prims.clone();

        let child = thread::spawn(move || {
            let prims;
            if id < NTHREADS-1 {
                prims = prim_get_numbers(&thread_smaller_prims, thread_start, thread_start + slice_size_per_thread);
            }
            else {
                prims = prim_get_numbers(&thread_smaller_prims, thread_start, end);
            }

            thread_tx.send((id, prims)).unwrap();
        });

        children.push(child);
    }

    let mut new_prims : Vec<Vec<u64>> = vec![Vec::new(); NTHREADS as usize];
    for _ in 0..NTHREADS {
        let nums = rx.recv().unwrap();
        new_prims[nums.0 as usize] = nums.1;
    }
    new_prims.sort();

    for child in children {
        child.join().expect("Failed to join a Child Thread.");
    }

    for mut add in new_prims {
        smaller_prims.append(&mut add);
    }
}

pub fn prim_count_trivial(end: u64) -> Vec<u64>{
    let mut smaller_prims: Vec<u64> = Vec::new();
    for num in 2..end {
        if prim_check_single_number(&smaller_prims, &num) {
            // println!("New prime: {}", &num);
            smaller_prims.push(num);
        }
    }
    smaller_prims
}

pub fn prim_count(end: u64) -> Vec<u64> {
    if end <= 11 {
        if end >= 2 {
            return prim_count_trivial(end)
        }
    }
    else
    {
        let mut smaller_prims: Vec<u64> = prim_count_trivial(12);
        let mut biggest_prim : u64 = 11;
        let mut biggest_prim_squared : u64 = biggest_prim * biggest_prim;
    
        while biggest_prim_squared < end {
            divide_among_threads(&mut smaller_prims, biggest_prim+1, biggest_prim_squared);
            biggest_prim = *smaller_prims.last().unwrap();
            biggest_prim_squared = biggest_prim * biggest_prim;
        }
        divide_among_threads(&mut smaller_prims, biggest_prim+1, end);
        return smaller_prims;
    }
    Vec::new()
}