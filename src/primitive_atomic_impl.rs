use super::atomic_step::AtomicStep;

impl AtomicStep for usize {
    fn atomic_step(&self) -> Self {
        1
    }
}

impl AtomicStep for u8 {
    fn atomic_step(&self) -> Self {
        1
    }
}

impl AtomicStep for u32 {
    fn atomic_step(&self) -> Self {
        1
    }
}

impl AtomicStep for u64 {
    fn atomic_step(&self) -> Self {
        1
    }
}

impl AtomicStep for u128 {
    fn atomic_step(&self) -> Self {
        1
    }
}

impl AtomicStep for isize {
    fn atomic_step(&self) -> Self {
        1
    }
}

impl AtomicStep for i8 {
    fn atomic_step(&self) -> Self {
        1
    }
}

impl AtomicStep for i32 {
    fn atomic_step(&self) -> Self {
        1
    }
}

impl AtomicStep for i64 {
    fn atomic_step(&self) -> Self {
        1
    }
}

impl AtomicStep for i128 {
    fn atomic_step(&self) -> Self {
        1
    }
}