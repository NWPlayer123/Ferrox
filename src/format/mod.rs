pub mod dol;
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Permissions: u32 {
        const READ = 1 << 0;
        const WRITE = 1 << 1;
        const EXECUTE = 1 << 2;
        const UNINITIALIZED = 1 << 3;
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
}

pub trait ValidSegmentSize: sealed::Sealed + Copy + Into<u64> {}
impl ValidSegmentSize for u16 {}
impl ValidSegmentSize for u32 {}
impl ValidSegmentSize for u64 {}

#[derive(Debug)]
pub struct Segment<T: ValidSegmentSize> {
    /// The virtual address this `Segment` starts at
    pub address: T,
    /// The size in bytes that this `Segment` takes up
    pub size: T,
    /// The file offset this Segment's data is at, if not [`Permissions::UNINITIALIZED`]
    // TODO: make this more flexible once we can store multiple binaries and modify existing code?
    pub offset: T,
    /// The permissions this `Segment` is tied to
    pub permissions: Permissions,
}
