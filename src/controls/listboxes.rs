use winapi::shared::windef::HWND;
use crate::window::winbuilder::Window;
use crate::controls::Point;
use crate::id_store::Id;
use super::controls::{Control,ControlType,Ctrl};
use super::winbuilder::WinAppBuilder;
#[derive(Clone)]
pub struct Listbox{
id : Id,label: String,point: Point, width: i32, height: i32
}
impl Control for Listbox{
    fn id(&self)->Id{
        self.id
    }
    fn place(&self, win: HWND){
       self.create( ControlType::StdControl(Ctrl::Listbox) , win, &self.label, self.id as i32, Point::new(self.point.x,self.point.y), self.width, self.height);
    }
    
    /* fn new(app: &mut WinAppBuilder)->Self{
        Button{id,label: label.to_string(), point, width, height}
    } */
}
impl Listbox{
    pub fn new(app: &mut WinAppBuilder, label: &str, point: Point, width: i32, height: i32)->Self{
        let id = app.new_id();
        let bt = Listbox{id,label: label.to_string(), point, width, height};
        app.add_control(Box::new(bt.clone()));
        //let vc = Button::create(self.hwnd(), label, id as i32, point, width, height);
        bt
    }
    
}
impl WinAppBuilder{
    pub fn add_listbox(&mut self,label: &str,point: Point, width: i32, height: i32)->Listbox{
        let id = self.new_id();
        let bt = Listbox{id,label: label.to_string(), point, width, height};
        //bt.add_items(self.hwnd(), &["blue"]);
        self.add_control(Box::new(bt.clone()));
        //let vc = Button::create(self.hwnd(), label, id as i32, point, width, height);
        bt
    }
}