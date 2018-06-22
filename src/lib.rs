fn bubble_sort(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut l = vec.len();
    let mut a: i32;
    let mut n: i32;
    let mut ap: usize;
    let mut np: usize;

    for _ in 0..l {
        println!("{:?}", &vec);

        for i in 1..l {
            ap = l - i;
            np = ap - 1;
            a = vec[ap];
            n = vec[np];

            if a < n {
                vec[ap] = n;
                vec[np] = a;
            }
        }

        l = l - 1;
    }

    vec
}
