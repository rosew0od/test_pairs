A simple testing library that allows you to test pairs.
Example:
``` rust
use test_pairs::test_pairs;

let pairs: Vec<(u8, u8)> = [
    (0x00, 0x00)
    (0x0F, 0xF0)
    (0xF0, 0x0F)
    (0xFF, 0xFF)
]

test_pairs(
    &pairs,
    |a, b| {
        a.to_le()
    },
    |b, a| {
        b.to_be()
    }
)
```
