use std::collections::HashMap;

fn main() {
let numbers: [i32; 14] = [1, 1, 2, 10, 3, 3, 3, 66, 0, 2, 3, 23, 2, 3];
let mut counts: HashMap<i32, i32> = HashMap::new();
for num in numbers {
	if counts.contains_key(&num){
		counts.insert(num.clone(), counts[&num]+1);
	}
	else{
		counts.insert(num, 1);
	}
}

for (key, value) in &counts {
	println!("{}: {}", key, value);
	}
}
