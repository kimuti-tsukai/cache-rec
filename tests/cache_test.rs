
#[cache_rec::cacher]
fn fact(i: i32) -> i32 {
    if i == 0 {
        1
    } else {
        i * fact(i-1)
    }
}

#[test]
fn fact_test() {
    dbg!(fact(5));
}

#[cache_rec::global_cacher]
fn fact_global(i: i32) -> i32 {
    if i == 0 {
        1
    } else {
        i * fact(i-1)
    }
}

#[cache_rec::global_cacher]
fn fact_global2(i: i32) -> i32 {
    if i == 0 {
        1
    } else {
        i * fact(i-1)
    }
}

#[test]
fn fact_global_test() {
    dbg!(fact_global(8));
    dbg!(fact_global2(3));
}
