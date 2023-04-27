use crate::branch::Branch;

struct Session {
    branch: Option<Branch>
}

impl Session{
    pub fn new() -> Self{
        Self {
            branch: None
        }
    }
}
