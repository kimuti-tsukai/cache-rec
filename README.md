# Memoization Recursion

## Usage
```rs
#[cache_rec::cache]
fn fact(i: i32) -> i32 {
    if i == 0 {
        1
    } else {
        i * fact(i-1)
    }
}
```

```rs
#[cache_rec::cache_global]
fn fact(i: i32) -> i32 {
    if i == 0 {
        1
    } else {
        i * fact(i-1)
    }
}
```
