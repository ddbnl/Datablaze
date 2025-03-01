

pub trait Datastore {
    fn insert(&mut self);
    fn delete(&mut self);
    fn create(&mut self);
    fn update(&mut self);
    fn select(&self);
}