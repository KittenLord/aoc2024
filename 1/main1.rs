use std::fs;

fn main() {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't find the file");
    let lines = contents.split_terminator("\n");

    let mut left : Vec<i32> = Vec::new();
    let mut right : Vec<i32> = Vec::new();
    for line in lines {
	let mut split = line.split_whitespace();
	let l = split.next().expect("Bad input");
	let r = split.next().expect("Bad input");
	let li : i32 = l.parse::<i32>().expect("Bad input");
	let ri : i32 = r.parse::<i32>().expect("Bad input");
	left.push(li);
	right.push(ri);
    }

    left.sort();
    right.sort();

    let mut acc = 0;
    for (a, b) in left.iter().zip(right.iter()) {
	acc += (a - b).abs();
    }

    println!("Result: {acc}");
}
