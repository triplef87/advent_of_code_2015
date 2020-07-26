#[derive(Debug)]
pub struct Wire {
    pub state: WireType,
    pub val: u16,
    pub src_a: String,
    pub src_b: String,
    pub a_val: (u16, bool),
    pub b_val: (u16, bool),
    pub command: Gate,
    pub dest: Vec<String>
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WireType {
    Wait,
    Signal
}

#[derive(Debug, Copy, Clone)]
pub enum Gate {
    Not,
    And,
    Or,
    Lshift,
    Rshift,
    Give
}

impl Wire {
    pub fn new() -> Wire {
        Wire {
            state: WireType::Wait,
            val: 0,
            src_a: "".to_string(),
            src_b: "".to_string(),
            a_val: (0, false),
            b_val: (0, false),
            command: Gate::Give,
            dest: Vec::new()
        }
    }

    pub fn get_dest(&self) -> &Vec<String> {
        &self.dest 
    }

    pub fn get_wire(&self) -> Wire {
        Wire {
            state: self.state,
            val: self.val,
            src_a: self.src_a.to_string(),
            src_b: self.src_b.to_string(),
            a_val: self.a_val,
            b_val:self.b_val,
            command: self.command,
            dest: self.dest.to_vec()
        }
    }
}