use pointer_identity::{Pointer, PointerIdentity};
use std::{
    cmp::Ordering,
    collections::{hash_map::DefaultHasher, BTreeSet, HashSet},
    hash::{Hash, Hasher},
    path::PathBuf,
    rc::Rc,
    sync::Arc,
};
use test_strategy::*;

#[derive(Arbitrary, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Struct {
    number: i64,
    string: String,
    vector: Vec<(usize, PathBuf)>,
}

fn hash_sum<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn test_identity<T: Pointer>(value: T) {
    let value = PointerIdentity::new(value);

    // make sure partialeq + eq work
    assert!(value.eq(&value));

    // make sure partialord + ord works
    assert_eq!(value.cmp(&value), Ordering::Equal);
    assert_eq!(value.partial_cmp(&value), Some(Ordering::Equal));

    // make sure hashes are equal
    assert_eq!(hash_sum(&value), hash_sum(&value));
}

#[test]
fn can_compare_identity() {
    // smart pointers
    test_identity(Rc::new(0u64));
    test_identity(Arc::new(0u64));
    test_identity(Box::new(0u64));

    // arrays
    test_identity(vec![0u64]);
    test_identity(&[0u64] as &[u64]);
    test_identity(&[0u64] as &[u64; 1]);

    // other
    #[cfg(feature = "bytes")]
    test_identity(bytes::Bytes::from(vec![0]));
}

fn test_cloned_identity<T: Pointer + Clone>(value: T) {
    // get value and a clone
    let value = PointerIdentity::new(value);
    let clone = value.clone();

    // make sure that partialeq + eq still work
    assert!(value.eq(&value));

    // make sure that partialord + ord still work
    assert_eq!(value.cmp(&clone), Ordering::Equal);
    assert_eq!(value.partial_cmp(&clone), Some(Ordering::Equal));

    // make sure hash sum ist still the same
    assert_eq!(hash_sum(&value), hash_sum(&clone));
}

#[test]
fn can_compare_cloned_identity() {
    // smart pointers
    test_cloned_identity(Rc::new(0u64));
    test_cloned_identity(Arc::new(()));

    // arrays
    test_cloned_identity(&[] as &[()]);
    test_cloned_identity(&[] as &[(); 0]);

    // other
    #[cfg(feature = "bytes")]
    test_cloned_identity(bytes::Bytes::from(vec![0]));
}

fn test_different<T: Pointer>(left: T, right: T) {
    let left = PointerIdentity::new(left);
    let right = PointerIdentity::new(right);

    // make sure partialeq + eq work
    assert!(!left.eq(&right));

    // make sure partialord + ord works
    assert!(left.cmp(&right) != Ordering::Equal);
    assert!(left.partial_cmp(&right).unwrap() != Ordering::Equal);

    // make sure hashes are equal
    assert!(hash_sum(&left) != hash_sum(&right));
}

#[proptest]
fn can_compare_different(left: u64, right: u64) {
    // smart pointers
    test_different(Rc::new(left), Rc::new(right));
    test_different(Arc::new(left), Arc::new(right));
    test_different(Box::new(left), Box::new(right));

    // arrays
    test_different::<&[u64]>(&[left], &[right]);
    test_different::<&[u64; 1]>(&[left], &[right]);
}

fn test_store<T: Pointer + Clone>(values: Vec<T>) {
    let mut hash_set = HashSet::new();
    let mut btree_set = BTreeSet::new();

    for value in values.iter() {
        hash_set.insert(PointerIdentity::new(value.clone()));
        btree_set.insert(PointerIdentity::new(value.clone()));
    }

    assert_eq!(hash_set.len(), values.len());
    assert_eq!(btree_set.len(), values.len());
}

#[proptest]
fn can_store(numbers: Vec<u64>) {
    test_store::<Rc<u64>>(numbers.iter().copied().map(Rc::new).collect());
    test_store::<Arc<u64>>(numbers.iter().copied().map(Arc::new).collect());
}

fn test_methods<T: Pointer + Clone + Eq + std::fmt::Debug>(value: T) {
    // get value and a clone
    let mut pointer = PointerIdentity::new(value.clone());

    // tests deref + deref mut
    assert_eq!(&*pointer, &value);
    *pointer = value.clone();

    assert_eq!(pointer.inner(), &value);
    assert_eq!(pointer.into_inner(), value);
}

#[test]
fn can_compare_methods() {
    // smart pointers
    test_methods(Rc::new(0u64));
    test_methods(Arc::new(0u64));
    test_methods(Box::new(0u64));

    // arrays
    test_methods(&[] as &[()]);
    test_methods(&[] as &[(); 0]);

    // other
    #[cfg(feature = "bytes")]
    test_methods(bytes::Bytes::from(vec![0]));
}
