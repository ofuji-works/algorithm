use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).ok();
    let mut vec: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();

    for i in 0..vec.len() - 1 {
        let mut is_need_swap = false;
        let mut min_index: usize = i;

        for u in i + 1..vec.len() {
            if vec[min_index] > vec[u] {
                min_index = u;
                is_need_swap = true;
            }
        }

        if is_need_swap {
            vec.swap(i, min_index);
        }
    }

    println!("{:?}", vec);
}
