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

#[cfg(test)]
mod test {
    use super::*;

    fn setup_u32() -> impl Iterator<Item=u32> {
        (0..10).chain(20..31).chain(45..50).chain(60..61)
    }

    #[test]
    fn test_u32_atomic_group() {
        let mut group = atomic_group(setup_u32());
        assert_eq!(group, Vec::from([(0, 9), (20, 30), (45,49), (60, 60)]));

        group = atomic_group((0..1));
        assert_eq!(group, Vec::from([(0, 0)]));
    }
}