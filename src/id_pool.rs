pub mod st;

pub use u32 as Id;
pub trait IdPool {
    fn next(&mut self) -> Option<Id>;
    fn release(&mut self, id: Id);
    fn reset(&mut self);
}
