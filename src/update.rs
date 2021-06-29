pub trait Updatable<T>{
    fn update(&mut self, message: T);
}