use std::collections::HashMap;

use rug::Integer;

fn main() {
    let num: u64 = 100_000;
    let t = std::thread::Builder::new()
        .stack_size(num as usize * 0xFFFF)
        .spawn(move || {
            let int = Integer::from(100_000);
            println!("fib({}) = {}", int, fib(int.clone()));
        });

    let handler = t.unwrap();
    handler.join().unwrap();
}

fn fib(n: Integer) -> Integer {
    let mut map: HashMap<Integer, Integer> = HashMap::new();
    fib_m(n, &mut map)
}

fn fib_m(n: Integer, map: &mut HashMap<Integer, Integer>) -> Integer {
    let ret: Integer = Integer::from(1);
    if n <= 2 {
        return ret;
    }

    if map.contains_key(&n) {
        return map[&n].clone();
    }

    let ret = fib_m(&n - Integer::from(1), map) + fib_m(&n - Integer::from(2), map);
    map.insert(n, ret.clone());
    ret
}
