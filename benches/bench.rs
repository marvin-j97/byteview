use bytes::BufMut;
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
    let mut group = c.benchmark_group("ctor_short");

    let value = b"abcdefabcdef";

    group.bench_function("Arc'd slice", |b| {
        b.iter(|| std::sync::Arc::from(value));
    });

    group.bench_function("tokio::Bytes", |b| {
        b.iter(|| bytes::Bytes::copy_from_slice(value));
    });

    group.bench_function("ByteView", |b| {
        b.iter(|| ByteView::from(*value));
    });
}

fn ctor_long(c: &mut Criterion) {
    let mut group = c.benchmark_group("ctor_long");

    let value = b"abcdefabcdefabcdefabcdefabcdefabcdef";

    group.bench_function("Arc'd slice", |b| {
        b.iter(|| std::sync::Arc::from(value));
    });

    group.bench_function("tokio::Bytes", |b| {
        b.iter(|| bytes::Bytes::copy_from_slice(value));
    });

    group.bench_function("ByteView", |b| {
        b.iter(|| ByteView::from(*value));
    });
}

// Simulates `lsm-tree`-like deserializing of KV values
fn ctor_from_reader(c: &mut Criterion) {
    use std::sync::Arc;

    let mut group = c.benchmark_group("ctor_long from reader");

    let value = b"abcdefabcdefabcdefabcdefabcdefabcdef";

    group.bench_function("Arc'd slice", |b| {
        b.iter(|| {
            let mut c = Cursor::new(value);
            let mut v = vec![0; value.len()];
            c.read_exact(&mut v).unwrap();
            let x: Arc<[u8]> = v.into();
            x
        });
    });

    group.bench_function("Arc'd slice - preallocated", |b| {
        b.iter(|| {
            let mut c = Cursor::new(value);

            let v = vec![0; value.len()];
            let mut v: Arc<[u8]> = v.into();

            let builder = Arc::get_mut(&mut v).unwrap();
            c.read_exact(builder).unwrap();

            v
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
            x
        });
    });

    group.bench_function("tokio::Bytes (zeroed)", |b| {
        b.iter(|| {
            let mut c = Cursor::new(value);
            let mut builder = bytes::BytesMut::zeroed(value.len());
            c.read_exact(&mut builder).unwrap();
            builder.freeze()
        });
    });

    group.bench_function("tokio::Bytes (writer)", |b| {
        b.iter(|| {
            let mut c = Cursor::new(value).take(value.len() as u64);
            let mut builder = bytes::BytesMut::with_capacity(value.len()).writer();
            let n = std::io::copy(&mut c, &mut builder).unwrap();
            assert!(n == value.len() as u64);
            builder.into_inner().freeze()
        });
    });

    group.bench_function("tokio::Bytes (unsafe)", |b| {
        b.iter(|| {
            let mut c = Cursor::new(value);
            let mut builder = bytes::BytesMut::with_capacity(value.len());
            unsafe {
                builder.set_len(value.len());
            }
            c.read_exact(&mut builder).unwrap();
            builder.freeze()
        });
    });

    group.bench_function("ByteView::from_reader", |b| {
        b.iter(|| {
            let mut c = Cursor::new(value);
            ByteView::from_reader(&mut c, value.len()).unwrap()
        });
    });
}

// Simulates `value-log`-like deserializing of blobs
fn ctor_from_reader_blob(c: &mut Criterion) {
    use std::sync::Arc;

    let mut group = c.benchmark_group("ctor_blob from reader");

    let value = b"abcdefabcdefabcdefabcdefabcdefabcdef".repeat(1_000);

    group.bench_function("Arc'd slice", |b| {
        b.iter(|| {
            let mut c = Cursor::new(&value);
            let mut v = vec![0; value.len()];
            c.read_exact(&mut v).unwrap();
            let x: Arc<[u8]> = v.into();
            x
        });
    });

    group.bench_function("Arc'd slice - preallocated", |b| {
        b.iter(|| {
            let mut c = Cursor::new(&value);

            let v = vec![0; value.len()];
            let mut v: Arc<[u8]> = v.into();

            let builder = Arc::get_mut(&mut v).unwrap();
            c.read_exact(builder).unwrap();
            v
        });
    });

    group.bench_function("ByteView::with_size", |b| {
        b.iter(|| {
            let mut c = Cursor::new(&value);

            let mut x = ByteView::with_size(value.len());
            {
                let mut builder = x.get_mut().unwrap();
                c.read_exact(&mut builder).unwrap();
            }
            x
        });
    });

    group.bench_function("tokio::Bytes (zeroed)", |b| {
        b.iter(|| {
            let mut c = Cursor::new(&value);
            let mut builder = bytes::BytesMut::zeroed(value.len());
            c.read_exact(&mut builder).unwrap();
            builder.freeze()
        });
    });

    group.bench_function("tokio::Bytes (writer)", |b| {
        b.iter(|| {
            let mut c = Cursor::new(&value).take(value.len() as u64);
            let mut builder = bytes::BytesMut::with_capacity(value.len()).writer();
            let n = std::io::copy(&mut c, &mut builder).unwrap();
            assert!(n == value.len() as u64);
            builder.into_inner().freeze()
        });
    });

    group.bench_function("tokio::Bytes (unsafe)", |b| {
        b.iter(|| {
            let mut c = Cursor::new(&value);
            let mut builder = bytes::BytesMut::with_capacity(value.len());
            unsafe {
                builder.set_len(value.len());
            }
            c.read_exact(&mut builder).unwrap();
            builder.freeze()
        });
    });

    group.bench_function("ByteView::from_reader", |b| {
        b.iter(|| {
            let mut c = Cursor::new(&value);
            ByteView::from_reader(&mut c, value.len()).unwrap()
        });
    });
}

criterion_group!(
    benches,
    ctor_short,
    ctor_long,
    ctor_from_reader,
    ctor_from_reader_blob,
    eq_short,
    eq_long,
    cmp_short,
    cmp_long,
);
criterion_main!(benches);
