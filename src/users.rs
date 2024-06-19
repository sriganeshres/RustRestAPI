use std::fmt::{self, Debug};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]

pub struct User {
    pub name: String,
}

impl User {
    pub(crate) fn new(name: String) -> Self {
        User { name }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User {{ name: {:?} }}", self.name)
    }
}
