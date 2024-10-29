use std::cmp::PartialEq;
use std::fmt::Debug;

pub fn test_pairs<TA, TB, FA2B, FB2A>(pairs: &[(TA, TB)], fa2b: FA2B, fb2a: FB2A)
where
    TA: Debug + PartialEq + Clone,
    TB: Debug + PartialEq + Clone,
    FA2B: Fn(TA, TB) -> TB,
    FB2A: Fn(TB, TA) -> TA,
{
    for pair in pairs {
        assert_eq!(fa2b(pair.0.clone(), pair.1.clone()), pair.1.clone());
        assert_eq!(fb2a(pair.1.clone(), pair.0.clone()), pair.0.clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::test_pairs;
    #[test]
    fn adding() {
        let pairs = [(0, 1), (2, 3), (3, 4)];
        test_pairs(&pairs, |a, b| a + 1, |b, a| b - 1);
    }
}
