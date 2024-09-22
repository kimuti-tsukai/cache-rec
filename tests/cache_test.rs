
#[cache_rec::cacher]
fn hello(i: i32) -> i32 {
    if i == 0 {
        i
    } else {
        hello(i-1)
    }
}

#[test]
fn expand() {
    hello(100);
}
