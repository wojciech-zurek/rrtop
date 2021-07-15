pub trait Updatable<T>{
    fn update(&mut self, metric: T);
}