// Memory lane
pub mod common;

pub trait LaneId {
    fn id(&self) -> u32;
}

impl LaneId for u8 {
    fn id(&self) -> u32 {
        1
    }
}

impl LaneId for i8 {
    fn id(&self) -> u32 {
        2
    }
}

impl<T> LaneId for &[T]
where
    T: LaneId,
{
    fn id(&self) -> u32 {
        2
    }
}
