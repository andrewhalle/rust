fn calc_idx(i: usize) -> usize {
    (i + i + 20) % 4
}

fn main() {
    let ns = [2, 3, 5, 7];

    for i in 3..10 {
        println!("{}", ns[i]);
    }

    for i in 3..10 {
        println!("{}", ns[i % 4]);
    }

    for i in 3..10 {
        println!("{}", ns[i % ns.len()]);
    }

    for i in 3..10 {
        println!("{}", ns[calc_idx(i)]);
    }

    for i in 3..10 {
        println!("{}", ns[calc_idx(i) % 4]);
    }

    let mut ms = vec![1, 2, 3, 4, 5, 6];
    for i in 0..ms.len() {
        ms[i] *= 2;
    }
    assert_eq!(ms, vec![2, 4, 6, 8, 10, 12]);

    let mut ms = vec![1, 2, 3, 4, 5, 6];
    for i in 0..ms.len() {
        let x = &mut ms[i];
        *x *= 2;
    }
    assert_eq!(ms, vec![2, 4, 6, 8, 10, 12]);
}
