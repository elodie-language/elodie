#[derive(Clone, Copy, Debug)]
pub struct Argument(pub u64);

impl Argument {
    pub fn to_string(&self) -> String {
        format!("arg_{}", self.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Temp(pub u64);

impl Temp {
    pub fn to_string(&self) -> String {
        format!("temp_{}", self.0)
    }
}

pub(crate) struct Scope {
    pub next_arguments: Vec<Argument>,
    pub next_temps: Vec<Temp>,
}

impl Scope {
    pub(crate) fn new() -> Self {
        let mut result = Self {
            next_arguments: vec![],
            next_temps: vec![],
        };
        result.enter();
        result
    }

    pub(crate) fn enter(&mut self) {
        self.next_arguments.push(Argument(1));
        self.next_temps.push(Temp(1));
    }

    pub(crate) fn leave(&mut self) {
        self.next_arguments.pop().unwrap();
        self.next_temps.pop().unwrap();
    }

    pub(crate) fn push_argument(&mut self) -> Argument {
        let next_arg = self.next_arguments.last_mut().unwrap();
        let result = next_arg.clone();
        next_arg.0 += 1;
        result
    }

    pub(crate) fn push_temp(&mut self) -> Temp {
        let next_temp = self.next_temps.last_mut().unwrap();
        let result = next_temp.clone();
        next_temp.0 += 1;
        result
    }
}