# Memoization Recursion

## Usage
```rs
#[cache_rec::cache]
fn fib(i: usize) -> usize {
    if i == 0 || i == 1 {
        1
    } else {
        fib(i-1) + fib(i-2)
    }
}
```

```rs
#[cache_rec::cache_global]
fn fib_global(i: usize) -> usize {
    if i == 0 || i == 1 {
        1
    } else {
        fib_global(i-1) + fib_global(i-2)
    }
}
```
