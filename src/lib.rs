pub struct MemoryHog{
    increment: usize,
    limit: usize,
    mem_vector: Vec<u8>
}

impl MemoryHog{
    pub fn new(increment: usize, limit: usize) -> MemoryHog{
        MemoryHog { increment, limit, mem_vector: vec![0; 0] }
    }
    pub fn tick(&mut self) -> (){
        if ! self.saturated() {
            self.mem_vector.resize(self.mem_vector.len() + self.increment , 0);
        }
        log::info!("Vector len={}B cap={}B", self.mem_vector.len(), self.mem_vector.capacity());
    }
    pub fn saturated(&self) -> bool{
        self.mem_vector.len() >= self.limit
    }
}

