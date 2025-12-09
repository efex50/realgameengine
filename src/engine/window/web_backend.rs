use crate::window::InnerWindow;



pub struct WebWindow{
    canvas_id:Option<String>,
    title:String,
}

impl WebWindow{
    pub fn new(title:String) -> Self{
        return Self { canvas_id: None, title};
    }
}



impl InnerWindow for WebWindow{
    fn set_canvas_id(&mut self,canvas_id:String) {
        self.canvas_id = Some(canvas_id);
    }
    
    fn set_title(&mut self,title:String) {
        todo!()
    }
}