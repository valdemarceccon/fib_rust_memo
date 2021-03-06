use std::collections::HashMap;
use std::option::Option;

fn main() {
    
	let mut n = 1;
	while n <= 100 {
		println!("fib({}) = {}", n, fib(n));
		n=n+1;
	}

}

fn fib(n: u128) -> u128 {
	let mut m: HashMap<u128,u128> = HashMap::new();
	fib_m(n, &mut m)
}

fn fib_m(n: u128, m: &mut HashMap<u128, u128>) -> u128 {
	if n <= 2 {
		return 1
	}

	return match m.get(&n) {
		Option::Some(&v) => v,
		Option::None => {
			let v = fib_m(n - 1, m) + fib_m(n - 2, m);
			m.insert(n,v);
			v
		}
	};
}