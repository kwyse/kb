//! Find the intersection points of two lines

/// My own implementation to solve CRLS exercies 1.2-2 and 1.2-3
///
/// Accepts two functions that only intersect up to two times along the
/// positive x-axis. If these intersections happen before the implemented
/// bound, the x-axis values are returned.
pub fn find_intersection<A, B>(a: A, b: B) -> (Option<f64>, Option<f64>)
where
    A: Fn(f64) -> f64,
    B: Fn(f64) -> f64
{
    let mut results = Vec::with_capacity(2);
    let mut converging = false;
    let mut delta = 0.0;

    let mut i = 0.0;
    while i <= 100.0 {
        let di = (a(i) - b(i)).abs();

        if converging && di > delta {
            results.push(Some(i));
            converging = false;
        } else if di < delta {
            converging = true;
        }
        
        delta = di;
        i += 0.01;
    }

    (*results.get(0).unwrap_or(&None), *results.get(1).unwrap_or(&None))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_intersection() {
        let a = |_: f64| 2.0;
        let b = |_: f64| 4.0;
        let (first, second) = find_intersection(a, b);
        assert!(first.is_none());
        assert!(second.is_none());

        let a = |n: f64| 2.0 * n + 10.0;
        let b = |n: f64| n.powi(2);
        let (first, second) = find_intersection(a, b);
        assert!(first.is_some());
        assert!(first.unwrap() > 4.31);
        assert!(first.unwrap() < 4.33);
        assert!(second.is_none());

        let a = |n: f64| 8.0 * n.powi(2);
        let b = |n: f64| 64.0 * n * n.log2();
        let (first, second) = find_intersection(a, b);
        assert!(first.is_some());
        assert!(first.unwrap() > 1.1);
        assert!(first.unwrap() < 1.12);
        assert!(second.is_some());
        assert!(second.unwrap() > 43.56);
        assert!(second.unwrap() < 43.58);

        let a = |n: f64| 100.0 * n.powi(2);
        let b = |n: f64| n.exp2();
        let (first, second) = find_intersection(a, b);
        assert!(first.is_some());
        assert!(first.unwrap() > 0.1);
        assert!(first.unwrap() < 0.11);
        assert!(second.is_some());
        assert!(second.unwrap() > 14.32);
        assert!(second.unwrap() < 14.34);
    }
}
