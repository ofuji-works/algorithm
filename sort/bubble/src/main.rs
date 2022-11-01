fn main() {
    let mut vec = vec![39, 8, 39, 45, 35, 43, 11, 29, 41, 31];

    loop {
        let mut swapped = false;

        for i in 1..vec.len() {
            if vec[i - 1] > vec[i] {
                vec.swap(i - 1, i);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }

    println!("{:?}", vec);
}
