use std::io::Write;
use std::str::FromStr;


fn main() 
{
	let mut numbers:Vec<u64> = Vec::new();

	for arg in std::env::args().skip(1)
	{
		numbers.push(u64::from_str(&arg)
							.expect("error parsing argument"));
	}

	if numbers.len() == 0
	{
		writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap(); //prints to std error
		std::process::exit(1);  
	}

	let mut d = numbers[0];
	for m in &numbers[1..]{
		d = gcd(d,*m);
	}

	println!("The greatest common divisor of {:?} is {}", numbers, d);
/*
    println!("Hello, world!");
    println!("{0}, and {1} and {}",gcd(42,1024), 23);
    println!("{}",gcd(42,1));*/
}

fn gcd(mut n:u64, mut m:u64) -> u64
{
	assert!(n != 0 && m != 0);
	while m!=0
	{
		if m<n 
		{
			let t = m;
			m = n;
			n = t;
		}

		m = m%n;
	}

	n
}

#[test]
fn test_gcd()
{
	assert_eq!(gcd(14,15), 1);
	assert_eq!(gcd(42,1024), 2);
}