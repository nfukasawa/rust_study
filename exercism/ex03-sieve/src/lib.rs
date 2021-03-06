pub fn primes_up_to(limit: i32) -> Vec<i32> {
    let mut xs = (2..limit + 1).collect::<Vec<i32>>();
    let mut ps = Vec::<i32>::new();

    while let Some(&p) = xs.first().take() {
        ps.push(p);
        xs.retain(|&x| x % p != 0);
        if p * p > limit {
            ps.extend_from_slice(&xs);
            break;
        }
    }
    ps
}
