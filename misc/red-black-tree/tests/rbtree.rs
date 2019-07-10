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
fn test_stress() {
    let t = Instant::now();

    let mut tree = rbtree::RBTreeSet::new();
    (0..100000).for_each(|n| {
        assert!(!tree.contains(&n));
        assert!(tree.insert(n));
        assert!(tree.contains(&n));
        assert_eq!(n+1, tree.len());
    });

    println!("time: {:?}", t.elapsed());
}