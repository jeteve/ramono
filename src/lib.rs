use std::{fs::File, io::Error};

pub struct MemoryHog{
    increment: usize,
    limit: usize,
    mem_vector: Vec<u8>
}

impl MemoryHog{
    pub fn new(increment: usize, limit: usize) -> MemoryHog{
        MemoryHog { increment, limit, mem_vector: vec![0; 0] }
    }
    pub fn tick(&mut self) {
        if ! self.saturated() {
            self.mem_vector.resize(self.mem_vector.len() + self.increment , 0);
            log::trace!("Memory alloc size={}B cap={}B", self.mem_vector.len(), self.mem_vector.capacity());
        } else {
            log::info!("Reached memory alloc saturation size={}B cap={}B", self.mem_vector.len(), self.mem_vector.capacity());
        }
    }
    pub fn saturated(&self) -> bool{
        self.mem_vector.len() >= self.limit
    }
}


pub struct FileHandlesHog{
    increment: usize,
    limit: usize,
    last_error: Option<Error>,
    files: Vec<File>
}

impl FileHandlesHog{
    pub fn new(increment: usize, limit: usize) -> FileHandlesHog{
        log::info!("Files will be generated in {}", std::env::temp_dir().to_string_lossy());
        FileHandlesHog{increment, limit, files: Vec::new(), last_error: None}
    }


    pub fn tick(&mut self) {
        if self.saturated() {
            log::info!("Reached file handle saturation N={} MAX={} ERR={:?}", self.files.len(), self.limit, self.last_error);
            return;
        }

        // Open increment files and exits if not possible.
        for _ in 0..self.increment {
            let rfile = tempfile::tempfile();
            if let Ok(f) = rfile {
                self.files.push(f);
            }else{
                self.last_error = Some(rfile.expect_err("Should be an error"));
                log::warn!("Error opening temp file: {:?}", self.last_error.as_ref().unwrap() );
                return;
            }
        }
        log::trace!("FileHandles len={} cap={}", self.files.len(), self.limit);
    }
    pub fn saturated(&self) -> bool{
        // Either the OS saturated
        self.last_error.is_some() || self.files.len() >= self.limit
    }
}
