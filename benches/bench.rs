use criterion::{black_box, criterion_group, criterion_main, Criterion};
use thin_slice::ThinSlice;
use std::time::Duration;

fn cmp_short(c: &mut Criterion) {
    let mut group = c.benchmark_group("cmp short");
    group.measurement_time(Duration::from_secs(3));

    let x = nanoid::nanoid!(8);
    let y = nanoid::nanoid!(8);

    {
        /*  let strings = (0..10_000_000)
        .map(|_| std::sync::Arc::<[u8]>::from(nanoid::nanoid!(8).as_bytes()))
        .collect::<Vec<_>>(); */

        group.bench_function("Arc'd slice", |b| {
            b.iter(|| {
                /* let idx = rng.gen_range(0..strings.len());
                let x = &strings[idx];

                let idx = rng.gen_range(0..strings.len());
                let y = &strings[idx]; */

                let _ = black_box(x.cmp(&y));
            });
        });
    }

    {
        /*  let strings = (0..10_000_000)
        .map(|_| nanoid::nanoid!(8))
        .map(ThinSlice::from)
        .collect::<Vec<_>>(); */

        let x = ThinSlice::from(x);
        let y = ThinSlice::from(y);

        group.bench_function("ThinSlice", |b| {
            b.iter(|| {
                /*    let idx = rng.gen_range(0..strings.len());
                let x = &strings[idx];

                let idx = rng.gen_range(0..strings.len());
                let y = &strings[idx]; */

                let _ = black_box(x.cmp(&y));
            });
        });
    }
}

fn cmp_long(c: &mut Criterion) {
    let mut group = c.benchmark_group("cmp long");
    group.measurement_time(Duration::from_secs(3));

    //  let mut rng = rand::thread_rng();

    let x = nanoid::nanoid!(8);
    let y = nanoid::nanoid!(8);

    {
        /* let strings = (0..10_000_000)
        .map(|_| std::sync::Arc::<[u8]>::from(nanoid::nanoid!().as_bytes()))
        .collect::<Vec<_>>(); */

        group.bench_function("Arc'd slice", |b| {
            b.iter(|| {
                /* let idx = rng.gen_range(0..strings.len());
                let x = &strings[idx];

                let idx = rng.gen_range(0..strings.len());
                let y = &strings[idx]; */

                let _ = black_box(x.cmp(&y));
            });
        });
    }

    {
        /* let strings = (0..10_000_000)
        .map(|_| nanoid::nanoid!())
        .map(ThinSlice::from)
        .collect::<Vec<_>>(); */

        let x = ThinSlice::from(x);
        let y = ThinSlice::from(y);

        group.bench_function("ThinSlice", |b| {
            b.iter(|| {
                /* let idx = rng.gen_range(0..strings.len());
                let x = &strings[idx];

                let idx = rng.gen_range(0..strings.len());
                let y = &strings[idx]; */

                let _ = black_box(x.cmp(&y));
            });
        });
    }
}

fn eq_short(c: &mut Criterion) {
    let mut group = c.benchmark_group("eq short");
    group.measurement_time(Duration::from_secs(3));

    let x = nanoid::nanoid!(8);
    let y = nanoid::nanoid!(8);

    {
        /*  let strings = (0..10_000_000)
        .map(|_| std::sync::Arc::<[u8]>::from(nanoid::nanoid!(8).as_bytes()))
        .collect::<Vec<_>>(); */

        group.bench_function("Arc'd slice", |b| {
            b.iter(|| {
                /*  let idx = rng.gen_range(0..strings.len());
                let x = &strings[idx];

                let idx = rng.gen_range(0..strings.len());
                let y = &strings[idx]; */

                let _ = black_box(x.eq(&y));
            });
        });
    }

    {
        /*  let strings = (0..10_000_000)
        .map(|_| nanoid::nanoid!(8))
        .map(ThinSlice::from)
        .collect::<Vec<_>>(); */

        let x = ThinSlice::from(x);
        let y = ThinSlice::from(y);

        group.bench_function("ThinSlice", |b| {
            b.iter(|| {
                /*   let idx = rng.gen_range(0..strings.len());
                let x = &strings[idx];

                let idx = rng.gen_range(0..strings.len());
                let y = &strings[idx]; */

                let _ = black_box(x.eq(&y));
            });
        });
    }
}

fn eq_long(c: &mut Criterion) {
    let mut group = c.benchmark_group("eq long");
    group.measurement_time(Duration::from_secs(3));

    // let mut rng = rand::thread_rng();

    let x = nanoid::nanoid!(8);
    let y = nanoid::nanoid!(8);

    {
        /* let strings = (0..10_000_000)
        .map(|_| std::sync::Arc::<[u8]>::from(nanoid::nanoid!().as_bytes()))
        .collect::<Vec<_>>(); */

        group.bench_function("Arc'd slice", |b| {
            b.iter(|| {
                /*                 let idx = rng.gen_range(0..strings.len());
                let x = &strings[idx];

                let idx = rng.gen_range(0..strings.len());
                let y = &strings[idx]; */

                let _ = black_box(x.eq(&y));
            });
        });
    }

    {
        /*     let strings = (0..10_000_000)
        .map(|_| nanoid::nanoid!())
        .map(ThinSlice::from)
        .collect::<Vec<_>>(); */

        let x = ThinSlice::from(x);
        let y = ThinSlice::from(y);

        group.bench_function("ThinSlice", |b| {
            b.iter(|| {
                /*  let idx = rng.gen_range(0..strings.len());
                let x = &strings[idx];

                let idx = rng.gen_range(0..strings.len());
                let y = &strings[idx]; */

                let _ = black_box(x.eq(&y));
            });
        });
    }
}

fn ctor(c: &mut Criterion) {
    let mut group = c.benchmark_group("ctor long");

    group.bench_function("Arc'd slice", |b| {
        b.iter(|| {
            let _x: std::sync::Arc<[u8]> =
                std::sync::Arc::from(nanoid::nanoid!().clone().as_bytes());
        });
    });

    group.bench_function("ThinSlice", |b| {
        b.iter(|| {
            let _x = ThinSlice::from(nanoid::nanoid!());
        });
    });
}

criterion_group!(benches, eq_short, eq_long, cmp_short, cmp_long, ctor);
criterion_main!(benches);
