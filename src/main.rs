use std::collections::HashMap;
use rug::{Assign, Integer};

fn main() {
	let int = Integer::new() + 50;
	println!("{}",fib(int));
}

fn fib(n: Integer) -> Integer {
	let ret: Integer = Integer::new() + 1;
	if n <= 2 {
		return ret
	}

	fib(n.clone() - 1) + fib(n.clone() - 2)
}

