#[cache_rec::cache]
fn fib(i: usize) -> usize {
    if i == 0 || i == 1 {
        1
    } else {
        fib(i - 1) + fib(i - 2)
    }
}

#[test]
fn fib_test() {
    dbg!(fib(100));
    dbg!(fib(40));
}

#[cache_rec::cache_global]
fn fib_global(i: usize) -> usize {
    if i == 0 || i == 1 {
        1
    } else {
        fib_global(i - 1) + fib_global(i - 2)
    }
}

#[test]
fn fib_global_test() {
    dbg!(fib_global(410));
    dbg!(fib_global(710));
}

fn normal_fib(i: usize) -> usize {
    if i == 0 || i == 1 {
        1
    } else {
        normal_fib(i - 1) + normal_fib(i - 2)
    }
}

#[test]
fn benchmark() {
    fn bench<F: Fn() -> T, T>(f: F, n: u128) -> u128 {
        let mut s = 0;

        for _ in 0..n {
            let start = std::time::Instant::now();

            let _ = f();

            let elapsed = start.elapsed().as_millis();
            s += elapsed;
        }

        s / n
    }

    // First time
    dbg!(bench(|| normal_fib(40), 10));

    dbg!(bench(|| fib(40), 10));

    dbg!(bench(|| fib_global(40), 10));

    // Second time
    dbg!(bench(|| normal_fib(40), 10));

    dbg!(bench(|| fib(40), 10));

    dbg!(bench(|| fib_global(40), 10));
}
