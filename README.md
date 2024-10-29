A simple testing library that allows you to test pairs.
A simple example for testing that adding and subtracting are working:
``` rust
#[test]
fn adding() {
    let pairs = [
        (0, 1),
        (2, 3),
        (3, 4)
    ];
    test_pairs(
        &pairs,
        |a, b| a + 1,
        |b, a| b - 1
    );
}
```

If you only want to test turning a into b, but not b into a, you can simply return a:
``` rust
#[test]
fn adding() {
    let pairs = [
        (0, 1),
        (2, 3),
        (3, 4)
    ];
    test_pairs(
        &pairs,
        |a, b| a + 1,
        |b, a| a
    );
}
```
