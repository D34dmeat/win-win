use crate::id_store::Id;


pub trait Callback<T> where T: FnMut()->bool{
    
    fn run(&mut self)->bool;
}
pub type cllbck = Box<dyn FnMut(Id)->bool>;
pub struct Respond{}
/* impl<T> Callback<T= dyn FnMut> for Respond{

} */