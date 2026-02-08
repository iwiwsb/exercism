use rand::Rng;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

type SharedPool = Rc<RefCell<HashSet<String>>>;

/// A `RobotFactory` is responsible for ensuring that all robots produced by
/// it have a unique name. Robots from different factories can have the same
/// name.
pub struct RobotFactory {
    name_history: SharedPool,
}

pub struct Robot {
    name: String,
    name_history: SharedPool,
}

impl RobotFactory {
    pub fn new() -> Self {
        Self {
            name_history: Rc::new(RefCell::new(HashSet::new())),
        }
    }

    pub fn new_robot<R: Rng>(&mut self, rng: &mut R) -> Robot {
        Robot {
            name: Self::generate_robot_name(rng, &self.name_history),
            name_history: Rc::clone(&self.name_history),
        }
    }

    fn generate_robot_name<R: Rng>(rng: &mut R, name_history: &SharedPool) -> String {
        loop {
            let first_letter = rng.random_range('A'..='Z');
            let second_letter = rng.random_range('A'..='Z');
            let number = rng.random_range(0..=999);
            let name = format!("{}{}{:03}", first_letter, second_letter, number);
            if name_history.borrow_mut().insert(name.clone()) {
                return name;
            }
        }
    }
}

impl Robot {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset<R: Rng>(&mut self, rng: &mut R) {
        self.name_history.borrow_mut().remove(self.name());
        self.name = RobotFactory::generate_robot_name(rng, &self.name_history);
    }
}
