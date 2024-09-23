
#[cache_rec::cache]
fn fib(i: usize) -> usize {
    if i == 0 || i == 1 {
        1
    } else {
        fib(i-1) + fib(i-2)
    }
}

#[test]
fn fact_test() {
    dbg!(fib(30));
    dbg!(fib(60));
}

#[cache_rec::cache_global]
fn fib_global(i: usize) -> usize {
    if i == 0 || i == 1 {
        1
    } else {
        fib_global(i-1) + fib_global(i-2)
    }
}

#[cache_rec::cache_global]
fn fib_global2(i: usize) -> usize {
    if i == 0 || i == 1 {
        1
    } else {
        fib_global2(i-1) + fib_global2(i-2)
    }
}

#[test]
fn fact_global_test() {
    dbg!(fib_global(45));
    dbg!(fib_global2(50));
}
