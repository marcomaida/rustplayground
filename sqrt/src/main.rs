use std::io;

fn main() {
    println!("Choose a number");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed");

    let trimmed = input.trim();
    match trimmed.parse::<f64>()
    {
    	Ok(x) => 
    	{
    		println!("calculating root of {}...", x);
    		println!("result is {}", sqrt(x));
    	},
    	Err(..) => println!("fail")
    }
}

fn sqrt(n :f64) -> f64 
{
	let mut x = n;
	let mut y = 1.;

	for _i in 0..100000
	{
		x = (x + y) / 2.;
		y = n/x;
	}

	x
}
