
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
    fact(5);
}
