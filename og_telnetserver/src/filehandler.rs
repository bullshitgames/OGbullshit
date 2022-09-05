use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct FileHandler{
    _file : Option<File>,
    path : String,
}


impl FileHandler{

    pub fn new() -> FileHandler{
        FileHandler{
            _file : None,
            path : String::new(),
        } 
    }

    pub fn set_path(&mut self, path: String){
        self.path = path;
    }

    pub fn open(&mut self){
        self._file = Some(File::options()
            .append(true)
            .create(true)
            .open(self.path.clone())
            .expect("Cannot open file."));
    }

    pub fn write(&mut self, msg : String) {
        if self._file.is_none() {  return ; }

        self._file.as_ref().unwrap()
        .write_all(msg.as_bytes())
        .expect(format!("Failed while writing : {}", msg).as_str());
    }

    pub fn read_lines(&mut self) -> Vec<String>{
        if self._file.is_none() {  return Vec::new(); }

        let file = self._file.as_ref().unwrap();
        let reader = BufReader::new(file);

        let mut result = Vec::<String>::new();

        for line in reader.lines() {
            let line = line.expect("Failed to read line.");

            result.push(line);
        }

        result
    }

    pub fn close(&mut self) {self._file = None;}

    async fn run_logger(path : String){
        let mut fh = FileHandler::new();
        fh.set_path(path);


    }
}