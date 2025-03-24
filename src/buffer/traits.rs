pub trait Rolling<T> 
where
    T: Clone + Default,
{
    fn new(size: usize) -> Self;

    fn push(&mut self, value: T);

    fn get(&self, i: usize) -> Option<&T>;

    fn last(&self) -> Option<&T>;

    fn last_mut(&mut self) -> Option<&mut T>;

    fn first(&self) -> Option<&T>;

    fn len(&self) -> usize;

    fn size(&self) -> usize;

    fn raw(&self) -> &Vec<T>;

    fn last_removed(&self) -> &Option<T>;

    fn count(&self) -> usize;

    fn is_empty(&self) -> bool;
    
    fn to_vec(&self) -> Vec<T>;
}
