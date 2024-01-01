use std::io::{self, Write, StdoutLock};

pub struct ProgressHandler<'a> {
    handle: StdoutLock<'a>

}

impl ProgressHandler<'_> {
    pub fn new() -> Self{
        let stdout = io::stdout();
        let handle = stdout.lock();
        Self { handle }
    }
    pub fn write_progress(&mut self, prog: f64){
        self.handle.flush().unwrap(); // Flushes the out stream
        
        write!(self.handle, "\rProgress: {:.1}%", prog).unwrap();
    }
    pub fn finish(&mut self){
        write!(self.handle, "\r").unwrap();
    }
}
