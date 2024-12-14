use byteview::ByteView;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::{
    io::{Cursor, Read},
    time::Duration,
};

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
        .map(ByteView::from)
        .collect::<Vec<_>>(); */

        let x = ByteView::from(x);
        let y = ByteView::from(y);

        group.bench_function("ByteView", |b| {
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
        .map(ByteView::from)
        .collect::<Vec<_>>(); */

        let x = ByteView::from(x);
        let y = ByteView::from(y);

        group.bench_function("ByteView", |b| {
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
        .map(ByteView::from)
        .collect::<Vec<_>>(); */

        let x = ByteView::from(x);
        let y = ByteView::from(y);

        group.bench_function("ByteView", |b| {
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
        .map(ByteView::from)
        .collect::<Vec<_>>(); */

        let x = ByteView::from(x);
        let y = ByteView::from(y);

        group.bench_function("ByteView", |b| {
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

fn ctor_short(c: &mut Criterion) {
    let mut group = c.benchmark_group("ctor short");

    let value = b"abcdefabcdef";

    group.bench_function("Arc'd slice", |b| {
        b.iter(|| {
            let _x = std::sync::Arc::from(value);
        });
    });

    group.bench_function("ByteView", |b| {
        b.iter(|| {
            let _x = ByteView::from(*value);
        });
    });
}

fn ctor_long(c: &mut Criterion) {
    let mut group = c.benchmark_group("ctor ctor_long");

    let value = b"abcdefabcdefabcdefabcdefabcdefabcdef";

    group.bench_function("Arc'd slice", |b| {
        b.iter(|| {
            let _x = std::sync::Arc::from(value);
        });
    });

    group.bench_function("ByteView", |b| {
        b.iter(|| {
            let _x = ByteView::from(*value);
        });
    });
}

// Simulates `lsm-tree`-like deserializing of KV values
fn ctor_from_reader(c: &mut Criterion) {
    use std::sync::Arc;

    let mut group = c.benchmark_group("ctor long from reader");

    let value = b"abcdefabcdefabcdefabcdefabcdefabcdef";

    group.bench_function("Arc'd slice", |b| {
        b.iter(|| {
            let mut c = Cursor::new(value);
            let mut v = vec![0; value.len()];
            c.read_exact(&mut v).unwrap();
            let _x: Arc<[u8]> = v.into();
        });
    });

    group.bench_function("Arc'd slice - preallocated", |b| {
        b.iter(|| {
            let mut c = Cursor::new(value);

            let v = vec![0; value.len()];
            let mut v: Arc<[u8]> = v.into();

            let builder = Arc::get_mut(&mut v).unwrap();
            c.read_exact(builder).unwrap();
        });
    });

    group.bench_function("ByteView::with_size", |b| {
        b.iter(|| {
            let mut c = Cursor::new(value);

            let mut x = ByteView::with_size(value.len());
            {
                let mut builder = x.get_mut().unwrap();
                c.read_exact(&mut builder).unwrap();
            }
        });
    });

    group.bench_function("ByteView::from_reader", |b| {
        b.iter(|| {
            let mut c = Cursor::new(value);
            let _x = ByteView::from_reader(&mut c, value.len()).unwrap();
        });
    });
}

criterion_group!(
    benches,
    ctor_short,
    ctor_long,
    ctor_from_reader,
    eq_short,
    eq_long,
    cmp_short,
    cmp_long,
);
criterion_main!(benches);
