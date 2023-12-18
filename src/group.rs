use std::ops::Add;

use super::atomic_step::AtomicStep;

/// Group immediately sucessive elements in an iterator together.
/// Returns a vector of tuples containing the start and end of 
/// each group.
/// 
/// Iterator elements have to have an atomic step size so they can 
/// be discreetly checked.
pub fn atomic_group<T>(mut sequence: impl Iterator<Item=T>) -> Vec<(T, T)>
    where T: AtomicStep + Add<Output=T> + Copy
{
    let mut groups = Vec::new();
    let mut next = sequence.next();
    while let Some(start) = next {
        let mut current = start;
        loop {
            match sequence.next() {
                Some(value) => {
                    if current + current.atomic_step() == value {
                        current = value;
                    } else {
                        next = Some(value);
                        break;
                    }
                },
                None => { groups.push((start, current)); return groups; }
                }
        } 
        groups.push((start, current));
    }
    groups
}

pub fn _ungroup<T>(_grouped: Vec<(T, T)>) -> Vec<T> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    /// Module to test what happens if the atomic step size is varied.
    mod evens {
        use super::*;
        use std::cmp::Ord;
        use core::ops::Add;
        
        #[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Debug)]
        pub struct Evens(pub u32);

        impl Add for Evens {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl AtomicStep for Evens {
            fn atomic_step(&self) -> Self {
                Self(2)
            }
        }

    }

    fn positives() -> impl Iterator<Item=i32> {
        (0..10).chain(20..31).chain(45..50).chain(60..61)
    }

    fn negatives() -> impl Iterator<Item=i32> {
        (-50..-45).chain(-20..-5).chain(-3..10)
    }

    fn overlap() -> impl Iterator<Item=i32> {
        (-50..-45).chain(-48..-20).chain(0..10).chain(-5..10)
    }

    #[test]
    fn test_empty() {
        let mut group = atomic_group(0..1);
        assert_eq!(group, Vec::from([(0, 0)]));
        group = atomic_group(0..0);
        assert_eq!(group, Vec::new());
    }

    #[test]
    fn test_positives() {
        let group = atomic_group(positives());
        assert_eq!(group, Vec::from([(0, 9), (20, 30), (45,49), (60, 60)]));
        let chained_group = atomic_group(positives().chain(61..65));
        assert_eq!(chained_group, Vec::from([(0, 9), (20, 30), (45,49), (60, 64)]));
    }


    #[test]
    fn test_negatives() {
        let group = atomic_group(negatives());
        assert_eq!(group, Vec::from([(-50, -46), (-20, -6), (-3,9)]));
    }

    #[test]
    fn test_overlap() {
        let group = atomic_group(overlap());
        assert_eq!(group, Vec::from([(-50, -46), (-48, -21), (0, 9), (-5, 9)]));
    }

    #[test]
    fn larger_step() {
        use evens::Evens;
        let evens = Vec::from([Evens(2), Evens(4), Evens(6), Evens(10)]);
        let group = atomic_group(evens.into_iter());
        assert_eq!(group, Vec::from([( Evens(2), Evens(6) ), ( Evens(10), Evens(10) )]));
        let evens = Vec::from([Evens(2), Evens(4), Evens(6), Evens(4), Evens(6), Evens(8)]);
        let group = atomic_group(evens.into_iter());
        assert_eq!(group, Vec::from([( Evens(2), Evens(6) ), ( Evens(4), Evens(8) )]));

    }
}