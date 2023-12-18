// use num_traits::PrimInt;

pub trait AtomicStep: Ord + Eq {

    /// Get the smallest discrete interval of the type implementing
    /// the trait. This interval may be constant (i.e., natural numbers)
    /// or changing (i.e., fibonacci).
    /// 
    /// In the case of a changing interval, the next atomic step is 
    /// dictated by the current state of the object.
    fn atomic_step(&self) -> Self;
}
