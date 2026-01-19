

pub struct IdPool
{
    free: Vec<u16>,
    next: u16,
    max: u16,
}

impl IdPool
{
    pub fn new(max: u16) -> Self
    {
        Self {
            free: Vec::new(),
            next: 0,
            max,
        }
    }

    pub fn alloc(&mut self) -> Option<u16>
    {
        if let Some(id) = self.free.pop() {
            Some(id)
        } else if self.next < self.max {
            let id = self.next;
            self.next += 1;
            Some(id)
        } else {
            None
        }
    }

    pub fn free(&mut self, id: u16)
    {
        self.free.push(id);
    }
}