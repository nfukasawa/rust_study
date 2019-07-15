use rand;
use rbtree;
use std::time::Instant;
// run tests by
// $ rustup run nightly cargo test
#[test]
fn test_rbtree() {
    let mut tree = rbtree::RBTreeSet::new();

    assert!(tree.insert(100));
    assert!(tree.insert(50));
    assert!(tree.insert(20)); // LL
    assert!(tree.insert(10));
    assert!(tree.insert(30));
    assert!(tree.insert(15)); // LR
    assert!(tree.insert(25)); // RL, LR
    assert!(tree.insert(110));
    assert!(tree.insert(120)); // RR

    assert!(!tree.insert(100));

    assert!(tree.contains(&100));
    assert!(!tree.contains(&101));

    assert_eq!(9, tree.len());
}

#[test]
fn test_stress_worst() {
    let t = Instant::now();

    let mut tree = rbtree::RBTreeSet::new();
    (0..100000).for_each(|n| {
        assert!(!tree.contains(&n));
        assert!(tree.insert(n));
        assert!(tree.contains(&n));
        assert_eq!(n + 1, tree.len());
    });

    println!("test_stress_worst: time: {:?}", t.elapsed());
}

#[test]
fn test_stress_rand() {
    let v = (0..100000).map(|_| rand::random::<i32>());

    let t = Instant::now();

    let mut tree = rbtree::RBTreeSet::new();
    let mut c = 0;
    v.for_each(|x| {
        if !tree.contains(&x) {
            assert!(tree.insert(x));
            c += 1;
        }
        assert!(tree.contains(&x));
        assert_eq!(c, tree.len());
    });

    println!("test_stress_rand: time: {:?}", t.elapsed());
}
