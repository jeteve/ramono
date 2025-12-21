use std::sync::mpsc::channel;
use std::{fs::File, io::Error};

#[derive(Debug)]
pub struct CpuSecondsHog {
    increment: usize,
    limit: usize,
    counter: usize,
    tx_thread: Option<std::sync::mpsc::Sender<usize>>,
    busy_thread: Option<std::thread::JoinHandle<()>>,
}

impl Drop for CpuSecondsHog {
    fn drop(&mut self) {
        // Force the channel to hangup.
        let tx = std::mem::take(&mut self.tx_thread);
        drop(tx);
        // Move the thread back to me and wait for it (should finish on hangup)
        let thread = std::mem::take(&mut self.busy_thread);
        let v = thread.expect("The thread should be there").join();
        if v.is_err() {
            log::warn!("Error in CPU busy thread: {:?}", v.unwrap_err());
        }
    }
}

impl CpuSecondsHog {
    pub fn new(increment: usize, limit: usize) -> CpuSecondsHog {
        let (tx, rx) = channel::<usize>();
        let hog = CpuSecondsHog {
            increment,
            limit,
            tx_thread: Some(tx),
            counter: 0,
            busy_thread: Some(std::thread::spawn(move || {
                loop {
                    // Wait for a tick to consume CPU seconds.
                    let m = rx.recv();
                    if m.is_err() {
                        log::debug!("Channel hangup. CPU busy thread exiting");
                        break;
                    }
                    let busy_seconds = m.unwrap();
                    // We have received a tick. Get busy with the increment.
                    // Busy loop for increment seconds.
                    let start = std::time::Instant::now();
                    while start.elapsed().as_secs() < busy_seconds as u64 {}
                }
            })),
        };
        log::debug!("{:?}", hog);
        hog
    }
    pub fn tick(&mut self) {
        if !self.saturated() {
            // Ask the thread to consume increment CPU seconds.
            log::trace!("CPU counter={}", self.counter);
            let _ = self.tx_thread.as_ref().unwrap().send(self.increment);
            self.counter += self.increment;
        } else {
            log::info!("Reached CPU seconds saturation counter={}", self.counter);
        }
    }
    pub fn saturated(&self) -> bool {
        self.counter >= self.limit
    }
}

#[derive(Debug)]
pub struct MemoryHog {
    increment: usize,
    limit: usize,
    mem_vector: Vec<u8>,
}

impl MemoryHog {
    pub fn new(increment: usize, limit: usize) -> MemoryHog {
        let hog = MemoryHog {
            increment,
            limit,
            mem_vector: vec![0; 0],
        };
        log::debug!("{:?}", hog);
        hog
    }
    pub fn tick(&mut self) {
        if !self.saturated() {
            self.mem_vector
                .resize(self.mem_vector.len() + self.increment, 0);
            log::trace!(
                "Memory alloc size={}B cap={}B",
                self.mem_vector.len(),
                self.mem_vector.capacity()
            );
        } else {
            log::info!(
                "Reached memory alloc saturation size={}B cap={}B",
                self.mem_vector.len(),
                self.mem_vector.capacity()
            );
        }
    }
    pub fn saturated(&self) -> bool {
        self.mem_vector.len() >= self.limit
    }
}

#[derive(Debug)]
pub struct FileHandlesHog {
    increment: usize,
    limit: usize,
    last_error: Option<Error>,
    files: Vec<File>,
}

impl FileHandlesHog {
    pub fn new(increment: usize, limit: usize) -> FileHandlesHog {
        let hog = FileHandlesHog {
            increment,
            limit,
            files: Vec::new(),
            last_error: None,
        };
        log::debug!(
            "{:?}. Files will be generated in {}",
            hog,
            std::env::temp_dir().to_string_lossy()
        );
        hog
    }

    pub fn tick(&mut self) {
        if self.saturated() {
            log::info!(
                "Reached file handle saturation N={} MAX={} ERR={:?}",
                self.files.len(),
                self.limit,
                self.last_error
            );
            return;
        }

        // Open increment files and exits if not possible.
        for _ in 0..self.increment {
            let rfile = tempfile::tempfile();
            if let Ok(f) = rfile {
                self.files.push(f);
            } else {
                self.last_error = Some(rfile.expect_err("Should be an error"));
                log::warn!(
                    "Error opening temp file: {:?}",
                    self.last_error.as_ref().unwrap()
                );
                return;
            }
        }
        log::trace!("FileHandles len={} cap={}", self.files.len(), self.limit);
    }
    pub fn saturated(&self) -> bool {
        // Either the OS saturated
        self.last_error.is_some() || self.files.len() >= self.limit
    }
}
