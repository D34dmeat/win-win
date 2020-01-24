pub type Id = u32;

pub struct IdStore{
    current: Id
}
impl IdStore{
    pub fn new()->Self{IdStore{current:0}}
    pub fn next(&mut self)->Id{self.current+=1;self.current}
    pub fn current(&self)->Id{self.current}
}