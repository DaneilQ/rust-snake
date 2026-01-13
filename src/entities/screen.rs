pub struct Screen {
    pub width: i32,
    pub height: i32,
}

impl Screen {
    pub fn init(width:i32, height:i32) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn resize(&mut self, width: i32, height: i32){
        self.width = width;
        self.height = height;
    }
}
