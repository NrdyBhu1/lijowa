/// Face enum to set faces 
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Face {
    Smile,
    Annoyed,
    Angry,
    Confused,
}

/// implementation for face enum
#[allow(dead_code)]
impl Face {
    /// returns a usize based on current self value
    pub fn get_face_index(self) -> usize {
        match self {
            Face::Smile => 0,
            Face::Annoyed => 1,
            Face::Angry => 2,
            Face::Confused => 3,
        }
    }

    /// set current value to smile only if it is not smile
    pub fn smile(&mut self) {
        if *self != Face::Smile {
            *self = Face::Smile;
        }
    }

    /// set current value to annoyed only if it is not annoyed
    pub fn annoy(&mut self) {
        if *self != Face::Annoyed {
            *self = Face::Annoyed;
        }
    }

    /// set current value to angry only if it is not angry
    pub fn angry(&mut self) {
        if *self != Face::Angry {
            *self = Face::Angry;
        }
    }

    /// set current value to confused only if it is not confused
    pub fn confuse(&mut self) {
        if *self != Face::Confused {
            *self = Face::Confused;
        }
    }
}
