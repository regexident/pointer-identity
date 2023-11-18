use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pointer_identity::PointerIdentity;
use rand::prelude::*;
use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
    sync::Arc,
};

fn criterion_benchmark(c: &mut Criterion) {
    let key_length = 10;
    let read_multiplier = 10;

    let mut group = c.benchmark_group("BTreeMap");
    let mut rng = thread_rng();
    for i in [10000usize].into_iter() {
        let mut keys: Vec<Vec<u8>> = vec![];
        for _ in 0..i {
            let mut key = vec![0; key_length];
            rng.fill(&mut key[0..key_length]);
            keys.push(key);
        }

        let mut lookup = vec![];
        for _ in 0..(read_multiplier * i) {
            lookup.push(rng.gen_range(0..keys.len()));
        }

        {
            let keys: Vec<Rc<[u8]>> = keys.iter().map(|key| key.clone().into()).collect();
            let lookup: Vec<_> = lookup.iter().map(|i| keys[*i].clone()).collect();
            let map: BTreeMap<Rc<[u8]>, usize> =
                keys.into_iter().enumerate().map(|(x, i)| (i, x)).collect();
            group.bench_with_input(BenchmarkId::new("Rc<[u8]>", i), &i, |b, i| {
                b.iter(|| {
                    for value in lookup.iter() {
                        black_box(map.get(value).unwrap());
                    }
                })
            });
        }

        {
            let keys: Vec<PointerIdentity<Rc<[u8]>>> = keys
                .iter()
                .map(|key| PointerIdentity(key.clone().into()))
                .collect();
            let lookup: Vec<_> = lookup.iter().map(|i| keys[*i].clone()).collect();
            let map: BTreeMap<_, usize> =
                keys.into_iter().enumerate().map(|(x, i)| (i, x)).collect();
            group.bench_with_input(
                BenchmarkId::new("PointerIdentity<Rc<[u8]>>", i),
                &i,
                |b, i| {
                    b.iter(|| {
                        for value in lookup.iter() {
                            black_box(map.get(value).unwrap());
                        }
                    })
                },
            );
        }

        {
            let keys: Vec<PointerIdentity<Arc<[u8]>>> = keys
                .iter()
                .map(|key| PointerIdentity(key.clone().into()))
                .collect();
            let lookup: Vec<_> = lookup.iter().map(|i| keys[*i].clone()).collect();
            let map: BTreeMap<_, usize> =
                keys.into_iter().enumerate().map(|(x, i)| (i, x)).collect();
            group.bench_with_input(
                BenchmarkId::new("PointerIdentity<Arc<[u8]>>", i),
                &i,
                |b, i| {
                    b.iter(|| {
                        for value in lookup.iter() {
                            black_box(map.get(value).unwrap());
                        }
                    })
                },
            );
        }
    }
    group.finish();

    let mut group = c.benchmark_group("HashMap");
    let mut rng = thread_rng();
    for i in [10000usize].into_iter() {
        let mut keys: Vec<Vec<u8>> = vec![];
        for _ in 0..i {
            let mut key = vec![0; key_length];
            rng.fill(&mut key[0..key_length]);
            keys.push(key);
        }

        let mut lookup = vec![];
        for _ in 0..(read_multiplier * i) {
            lookup.push(rng.gen_range(0..keys.len()));
        }

        {
            let keys: Vec<Rc<[u8]>> = keys.iter().map(|key| key.clone().into()).collect();
            let lookup: Vec<_> = lookup.iter().map(|i| keys[*i].clone()).collect();
            let map: HashMap<Rc<[u8]>, usize> =
                keys.into_iter().enumerate().map(|(x, i)| (i, x)).collect();
            group.bench_with_input(BenchmarkId::new("Rc<[u8]>", i), &i, |b, i| {
                b.iter(|| {
                    for value in lookup.iter() {
                        black_box(map.get(value).unwrap());
                    }
                })
            });
        }

        {
            let keys: Vec<PointerIdentity<Rc<[u8]>>> = keys
                .iter()
                .map(|key| PointerIdentity(key.clone().into()))
                .collect();
            let lookup: Vec<_> = lookup.iter().map(|i| keys[*i].clone()).collect();
            let map: HashMap<_, usize> =
                keys.into_iter().enumerate().map(|(x, i)| (i, x)).collect();
            group.bench_with_input(
                BenchmarkId::new("PointerIdentity<Rc<[u8]>>", i),
                &i,
                |b, i| {
                    b.iter(|| {
                        for value in lookup.iter() {
                            black_box(map.get(value).unwrap());
                        }
                    })
                },
            );
        }

        {
            let keys: Vec<PointerIdentity<Arc<[u8]>>> = keys
                .iter()
                .map(|key| PointerIdentity(key.clone().into()))
                .collect();
            let lookup: Vec<_> = lookup.iter().map(|i| keys[*i].clone()).collect();
            let map: HashMap<_, usize> =
                keys.into_iter().enumerate().map(|(x, i)| (i, x)).collect();
            group.bench_with_input(
                BenchmarkId::new("PointerIdentity<Arc<[u8]>>", i),
                &i,
                |b, i| {
                    b.iter(|| {
                        for value in lookup.iter() {
                            black_box(map.get(value).unwrap());
                        }
                    })
                },
            );
        }
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
