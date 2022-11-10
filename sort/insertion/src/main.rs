use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut vec: Vec<u32> = input
        .split_whitespace()
        .map(|e| e.trim().parse().unwrap())
        .collect();

    for i in 0..vec.len() {
        for u in 0..i {
            if vec[u] > vec[i] {
                vec.swap(u, i);
            }
        }
    }

    println!("{:?}", vec);
}
