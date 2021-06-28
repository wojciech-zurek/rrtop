pub trait Updatable<T>{
    fn update(&mut self, v: T);
}