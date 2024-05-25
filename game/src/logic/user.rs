use rand::Rng;

/// Decision is an enum that holds the possible decisions an user can make.
#[derive(Clone, Copy)]
pub enum Decision {
    TakeItem { from: usize },
    Peep { from: usize },
    None,
}

impl Decision {
    pub fn rand_choose(rng: &mut impl Rng, from: usize) -> Decision {
        let decision = match rng.gen_range(0..2) {
            0 => Decision::TakeItem { from },
            1 => Decision::Peep { from },
            2 => Decision::None,
            _ => panic!("Invalid decision"),
        };
        decision
    }
}

/// User is a struct that holds the id of the user and the locker layout it has in mind at the latest accessed moment.
#[derive(Clone)]
pub struct User {
    pub id: usize,
    pub inmind_locker_state_idx: usize,
}

impl User {
    pub fn new(id: usize, locker_state_idx: usize) -> User {
        User {
            id: id,
            inmind_locker_state_idx: locker_state_idx,
        }
    }
}

/// UserCollection is a struct that holds a list of users.
#[derive(Clone)]
pub struct UserCollection {
    pub users: Vec<User>,
}

impl UserCollection {
    pub fn new(user_n: usize, locker_state_idx: usize) -> UserCollection {
        UserCollection {
            users: (0..user_n).map(|i| User::new(i, locker_state_idx)).collect(),
        }
    }

    /// Remove and Get the user by id (not idx)
    pub fn remove_by_id(&mut self, id: usize) {
        self.users.retain(|user| user.id != id);
    }

    /// Get the user by id (not idx)
    pub fn get_mut_by_id(&mut self, id: usize) -> Option<&mut User> {
        self.users.iter_mut().find(|user| user.id == id)
    }

    /// Check if the user collection is empty
    pub fn is_empty(&self) -> bool {
        self.users.is_empty()
    }
}
