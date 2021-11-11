pub struct QueryBuilder {
    pid: u32,
}

impl QueryBuilder {
    pub fn with_pid(&mut self, pid: u32) -> &Self {
        self.pid = pid;
        self
    }
}
