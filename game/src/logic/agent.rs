use rand::{rngs::StdRng, Rng};

use super::locker::Locker;

#[derive(Clone, Copy)]
pub enum Decision {
    TakeItem { from: usize },
    Peep { from: usize },
    None,
}

impl Decision {
    pub fn rand_choose(rng: &mut StdRng, from: usize) -> Decision {
        let decision = match rng.gen_range(0..2) {
            0 => Decision::TakeItem { from },
            1 => Decision::Peep { from },
            2 => Decision::None,
            _ => panic!("Invalid decision"),
        };
        decision
    }
}

#[derive(Clone)]
pub struct Agent {
    pub id: usize,
    pub inmind_locker: Locker,
}

impl Agent {
    pub fn new(id: usize, locker: Locker) -> Agent {
        Agent {
            id: id,
            inmind_locker: locker,
        }
    }
}

#[derive(Clone)]
pub struct AgentCollection {
    pub agents: Vec<Agent>,
}

impl AgentCollection {
    pub fn new(agent_n: usize, locker: Locker) -> AgentCollection {
        AgentCollection {
            agents: (0..agent_n).map(|i| Agent::new(i, locker.clone())).collect(),
        }
    }

    pub fn remove_by_id(&mut self, id: usize) {
        self.agents.retain(|agent| agent.id != id);
    }

    pub fn get_mut_by_id(&mut self, id: usize) -> Option<&mut Agent> {
        self.agents.iter_mut().find(|agent| agent.id == id)
    }

    pub fn is_empty(&self) -> bool {
        self.agents.is_empty()
    }
}
