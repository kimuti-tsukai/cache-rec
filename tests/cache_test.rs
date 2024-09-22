
#[cache_rec::cacher]
fn hello(i: i32) {
    println!("{}", i);
}

#[test]
fn expand() {
    hello(100);
}
