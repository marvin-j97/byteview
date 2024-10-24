/* use byteview::StrView;
/// a criterion benchmark that compares the performance of counting the number
/// of strings in an array that match a particular prefix between GermanString and a regular rust String
use criterion::{criterion_group, criterion_main, Criterion};

fn scan_prefix_long(c: &mut Criterion) {
    // generate a large array of random strings, half of which start with "t"
    let strings: Vec<String> = (0..10000000)
        .map(|i| {
            if i % 2 == 0 {
                "thompson is a bad bad dog".into()
            } else {
                "routers is full of bugs".into()
            }
        })
        .collect();
    let prefix = "thom";

    let expected = strings.iter().filter(|s| s.starts_with(prefix)).count();

    let mut group = c.benchmark_group("scan_prefix_long");
    group.bench_function("rust_string", |b| {
        b.iter(|| {
            let mut count = 0;
            for s in strings.iter() {
                if s.starts_with(prefix) {
                    count += 1;
                }
            }
            assert_eq!(count, expected);
        });
    });

    let strings: Vec<StrView> = strings.into_iter().map(|s| s.into()).collect();
    group.bench_function("german_string", |b| {
        b.iter(|| {
            let mut count = 0;
            for s in strings.iter() {
                if s.starts_with(prefix) {
                    count += 1;
                }
            }
            assert_eq!(count, expected);
        });
    });
    group.finish();
}

fn scan_prefix_short(c: &mut Criterion) {
    // generate a large array of random strings, half of which start with "t"
    let strings: Vec<String> = (0..10000000)
        .map(|i| {
            if i % 2 == 0 {
                "thompson".into()
            } else {
                "routers".into()
            }
        })
        .collect();
    let prefix = "tho";

    let expected = strings
        .iter()
        .filter(|s| {
            assert!(s.len() <= 12);
            s.starts_with(prefix)
        })
        .count();

    let mut group = c.benchmark_group("scan_prefix_short");
    group.bench_function("rust_string", |b| {
        b.iter(|| {
            let mut count = 0;
            for s in strings.iter() {
                if s.starts_with(prefix) {
                    count += 1;
                }
            }
            assert_eq!(count, expected);
        });
    });

    let strings: Vec<StrView> = strings.into_iter().map(|s| s.into()).collect();
    group.bench_function("german_string", |b| {
        b.iter(|| {
            let mut count = 0;
            for s in strings.iter() {
                if s.starts_with(prefix) {
                    count += 1;
                }
            }
            assert_eq!(count, expected);
        });
    });
    group.finish();
}

criterion_group! {
    name=benches;
    config = Criterion::default();
    targets=scan_prefix_long, scan_prefix_short
}
criterion_main!(benches);
 */

use byteview::StrView;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng as _;

const INPUT_LENGTHS: [usize; 6] = [4, 8, 12, 16, 32, 64];

fn cmp_random(c: &mut Criterion) {
    let mut group = c.benchmark_group("cmp-random");
    for len in INPUT_LENGTHS {
        group.bench_with_input(BenchmarkId::new("String", len), &len, |b, &len| {
            b.iter_batched_ref(
                || (random_string(len), random_string(len)),
                |(a, b)| a.cmp(&b),
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("StrView", len), &len, |b, &len| {
            b.iter_batched_ref(
                || {
                    (
                        StrView::try_from(random_string(len)).unwrap(),
                        StrView::try_from(random_string(len)).unwrap(),
                    )
                },
                |(a, b)| a.cmp(&b),
                criterion::BatchSize::SmallInput,
            )
        });
    }
}

fn cmp_same(c: &mut Criterion) {
    let mut group = c.benchmark_group("cmp-same");
    for len in INPUT_LENGTHS {
        group.bench_with_input(BenchmarkId::new("String", len), &len, |b, &len| {
            b.iter_batched_ref(
                || {
                    let s = random_string(len);
                    (s.clone(), s)
                },
                |(a, b)| a.cmp(&b),
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("StrView", len), &len, |b, &len| {
            b.iter_batched_ref(
                || {
                    let s = random_string(len);
                    (
                        StrView::try_from(s.as_str()).unwrap(),
                        StrView::try_from(s.as_str()).unwrap(),
                    )
                },
                |(a, b)| a.cmp(&b),
                criterion::BatchSize::SmallInput,
            )
        });
    }
}

fn eq_random(c: &mut Criterion) {
    let mut group = c.benchmark_group("eq-random");
    for len in INPUT_LENGTHS {
        group.bench_with_input(BenchmarkId::new("String", len), &len, |b, &len| {
            b.iter_batched_ref(
                || (random_string(len), random_string(len)),
                |(a, b)| a.eq(&b),
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("StrView", len), &len, |b, &len| {
            b.iter_batched_ref(
                || {
                    (
                        StrView::try_from(random_string(len)).unwrap(),
                        StrView::try_from(random_string(len)).unwrap(),
                    )
                },
                |(a, b)| a.eq(&b),
                criterion::BatchSize::SmallInput,
            )
        });
    }
}

fn eq_same(c: &mut Criterion) {
    let mut group = c.benchmark_group("eq-same");
    for len in INPUT_LENGTHS {
        group.bench_with_input(BenchmarkId::new("String", len), &len, |b, &len| {
            b.iter_batched_ref(
                || {
                    let s = random_string(len);
                    (s.clone(), s)
                },
                |(a, b)| a.eq(&b),
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("StrView", len), &len, |b, &len| {
            b.iter_batched_ref(
                || {
                    let s = random_string(len);
                    (
                        StrView::try_from(s.as_str()).unwrap(),
                        StrView::try_from(s.as_str()).unwrap(),
                    )
                },
                |(a, b)| a.eq(&b),
                criterion::BatchSize::SmallInput,
            )
        });
    }
}

/* fn cmp_random_mixed_types(c: &mut Criterion) {
    let mut group = c.benchmark_group("cmp-random-mixed-types");
    for len in INPUT_LENGTHS {
        group.bench_with_input(BenchmarkId::new("String", len), &len, |b, &len| {
            b.iter_batched_ref(
                || (random_string(len), random_string(len)),
                |(a, b)| a.cmp(&b),
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("StrView", len), &len, |b, &len| {
            b.iter_batched_ref(
                || {
                    (
                        StrView::try_from(random_string(len)).unwrap(),
                        random_string(len),
                    )
                },
                |(a, b)| PartialOrd::partial_cmp(a, b),
                criterion::BatchSize::SmallInput,
            )
        });
    }
}

fn cmp_same_mixed_types(c: &mut Criterion) {
    let mut group = c.benchmark_group("cmp-same-mixed-types");
    for len in INPUT_LENGTHS {
        group.bench_with_input(BenchmarkId::new("String", len), &len, |b, &len| {
            b.iter_batched_ref(
                || {
                    let s = random_string(len);
                    (s.clone(), s)
                },
                |(a, b)| a.cmp(&b),
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("StrView", len), &len, |b, &len| {
            b.iter_batched_ref(
                || {
                    let s = random_string(len);
                    (StrView::try_from(s.as_str()).unwrap(), s)
                },
                |(a, b)| PartialOrd::partial_cmp(a, b),
                criterion::BatchSize::SmallInput,
            )
        });
    }
}

fn eq_random_mixed_types(c: &mut Criterion) {
    let mut group = c.benchmark_group("eq-random-mixed-types");
    for len in INPUT_LENGTHS {
        group.bench_with_input(BenchmarkId::new("String", len), &len, |b, &len| {
            b.iter_batched_ref(
                || (random_string(len), random_string(len)),
                |(a, b)| a.eq(&b),
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("StrView", len), &len, |b, &len| {
            b.iter_batched_ref(
                || {
                    (
                        StrView::try_from(random_string(len)).unwrap(),
                        random_string(len),
                    )
                },
                |(a, b)| PartialEq::eq(a, b),
                criterion::BatchSize::SmallInput,
            )
        });
    }
}

fn eq_same_mixed_types(c: &mut Criterion) {
    let mut group = c.benchmark_group("eq-same-mixed-types");
    for len in INPUT_LENGTHS {
        group.bench_with_input(BenchmarkId::new("String", len), &len, |b, &len| {
            b.iter_batched_ref(
                || {
                    let s = random_string(len);
                    (s.clone(), s)
                },
                |(a, b)| a.eq(&b),
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("StrView", len), &len, |b, &len| {
            b.iter_batched_ref(
                || {
                    let s = random_string(len);
                    (StrView::try_from(s.as_str()).unwrap(), s)
                },
                |(a, b)| PartialEq::eq(a, b),
                criterion::BatchSize::SmallInput,
            )
        });
    }
} */

fn construct_empty(c: &mut Criterion) {
    let mut group = c.benchmark_group("construct-empty");
    group.bench_function(BenchmarkId::new("StrView", "copy"), |b| {
        b.iter_batched(
            String::new,
            |s| StrView::try_from(s.as_str()),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function(BenchmarkId::new("StrView", "move"), |b| {
        b.iter_batched(
            String::new,
            StrView::try_from,
            criterion::BatchSize::SmallInput,
        )
    });
}

fn construct_non_empty(c: &mut Criterion) {
    let mut group = c.benchmark_group("construct-non-empty");
    for len in INPUT_LENGTHS {
        group.bench_with_input(BenchmarkId::new("StrViewCopy", len), &len, |b, &len| {
            b.iter_batched(
                || random_string(len),
                |s| StrView::try_from(s.as_str()),
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("StrViewMove", len), &len, |b, &len| {
            b.iter_batched(
                || random_string(len),
                StrView::try_from,
                criterion::BatchSize::SmallInput,
            )
        });
    }
}

fn random_string(len: usize) -> String {
    let bytes = rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(len)
        .collect::<Vec<_>>();

    String::from_utf8(bytes).unwrap()
}

criterion_group!(
    benches,
    cmp_random,
    cmp_same,
    eq_random,
    eq_same,
    /*    cmp_random_mixed_types,
    cmp_same_mixed_types,
    eq_random_mixed_types,
    eq_same_mixed_types, */
    construct_empty,
    construct_non_empty,
);
criterion_main!(benches);
