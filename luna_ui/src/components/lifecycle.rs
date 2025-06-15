#[derive(Debug, Clone, Copy, Default)]
pub enum LifeCycle {
    #[default]
    Created,
    Mounted,
}

impl LifeCycle {
    pub fn next(&mut self) -> () {
        match self {
            LifeCycle::Created => *self = LifeCycle::Mounted,
            LifeCycle::Mounted => (),
        }
    } 
    pub fn is_mounted(&self) -> bool {
        matches!(self, LifeCycle::Mounted)
    }
    pub fn is_created(&self) -> bool {
        matches!(self, LifeCycle::Created)
    }
}