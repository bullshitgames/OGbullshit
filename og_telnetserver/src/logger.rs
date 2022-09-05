
use crate::filehandler::FileHandler;

const DEFAULT_LOG_PATH  : &str = "server_logs/log.txt";

pub struct WatchLogger{
    _filehandler: FileHandler,

}



impl WatchLogger{

    pub fn new() -> Self {
        Self{
            _filehandler: FileHandler::new(DEFAULT_LOG_PATH.to_string()),
        }
    }

    pub fn start(&mut self, start_msg : Option<String>) {
        self._filehandler.open();
        if start_msg.is_some(){
            self._filehandler.write(start_msg.unwrap());
        }
    }

    pub fn stop(&mut self, stop_msg : Option<String>) {
        if stop_msg.is_some(){
            self._filehandler.write(stop_msg.unwrap());
        }
        self._filehandler.close();
    }

    pub fn log(&mut self, msg : String) {
        let date_time = chrono::offset::Local::now();

        let time = format!("{:?} : ",date_time);

        self._filehandler.write(time + &msg + "\n");
    }

}

impl Drop for WatchLogger{
    fn drop(&mut self){
        self.stop(None);
    }
}