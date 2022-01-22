use crate::win_proc::Act;
use crate::id_store::Id;


pub trait Callback<T> where T: Fn(&Act)->(){
    
    fn run(&mut self);
}
//pub type Callb = Fn(Id,&Act)->();
//pub type cllbck = Box<dyn Callback<dyn Fn(&Act)>>;
pub type Cllbck = Box<dyn Fn(&Act)->()>;
pub struct Respond{}
/* impl<T> Callback<T= dyn FnMut> for Respond{

} */