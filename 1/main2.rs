use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't find the file");
    let lines = contents.split_terminator("\n");

    let mut map : HashMap<i32, (i32, i32)> = HashMap::new();

    for line in lines {
	let mut split = line.split_whitespace();
	let l = split.next().expect("Bad input");
	let r = split.next().expect("Bad input");
	let li : i32 = l.parse::<i32>().expect("Bad input");
	let ri : i32 = r.parse::<i32>().expect("Bad input");

	let (a, _) = map.entry(li).or_insert((0, 0));
	*a += 1;
	let (_, b) = map.entry(ri).or_insert((0, 0));
	*b += 1;
    }
nn
    let mut acc = 0;
    for (key, (a, b)) in map.into_iter() {
	acc += key * a * b;
    }
    
    println!("Result: {acc}");
}
