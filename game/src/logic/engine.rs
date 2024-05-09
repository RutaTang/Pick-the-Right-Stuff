use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

use crate::utils::shuffle::shuffle;

use super::shelf::Shelf;

//TODO:
// - [] Finish Prompting
// - [] Finish Interaction

enum Scene {
    Init,           // Start the game, tell the player the game instruction
    DecisionMaking, // Agent should make a decision to (1) take item (2) peep (3) or nothing
    Shuffling,      // Shuffle the deck
    Peeping,        // Agent peep the status of the monitor
    Predicting,     // The player predict the agent's action
    End,            // tell the final result, and game over
}

struct State {
    score: usize,
    shelf: Shelf,
    agent_decision: Option<AgentDecision>,
}

#[derive(Clone, Copy)]
enum AgentDecision {
    TakeItem { from: usize },
    Peep { from: usize },
}

pub fn start() {
    let mut rng = StdRng::seed_from_u64(0);

    // game settings/options
    let agent_n = 3;

    // init the game
    let shelf = Shelf::new(agent_n);
    let mut agent_inmind_shelves = vec![shelf.clone(); agent_n];
    let mut state = State {
        score: 0,
        agent_decision: None,
        shelf: shelf,
    };
    let mut scene = Scene::Init;
    loop {
        match scene {
            Scene::Init => {
                // init the game: init agents & items, shuffle the items
                // tell LLM the game instruction, and current status
                println!("Welcome to, Be a Warehouse Manager!");

                // change to shuffling state
                scene = Scene::Shuffling;
            }
            Scene::DecisionMaking => {
                // agent should make a decision to (1) take item (2) peep (3) or nothing
                let agent_idx = rng.gen_range(0..agent_n);
                let decision: u8 = rng.gen_range(1..=3);
                match decision {
                    // Take item
                    1 => {
                        state.agent_decision = Some(AgentDecision::TakeItem { from: agent_idx });
                    }
                    // Peep
                    2 => {
                        state.agent_decision = Some(AgentDecision::Peep { from: agent_idx });
                    }
                    // Do nothing
                    3 => {
                        state.agent_decision = None;
                    }
                    _ => {
                        panic!("Invalid decision");
                    }
                }
                // randomly change to one of the following states
                // 1. Shuffling (must if agent want to take the item)
                // 2. Peeping (must if agent want to peep the status of the monitor)
                // 3. Change to Shuffling or Peeping (if agent do nothing)
                match state.agent_decision {
                    Some(AgentDecision::TakeItem { .. }) => {
                        scene = Scene::Predicting;
                    }
                    Some(AgentDecision::Peep { .. }) => {
                        scene = Scene::Peeping;
                    }
                    None => match rng.gen_bool(0.5) {
                        true => {
                            scene = Scene::Shuffling;
                        }
                        false => {
                            scene = Scene::Peeping;
                        }
                    },
                }
            }
            Scene::Shuffling => {
                if let Some(AgentDecision::TakeItem { from }) = state.agent_decision {
                    // agent try to take the item, must shuffle the items
                    println!("Shuffling the deck...");
                    let agent_in_mind_shelf = &mut agent_inmind_shelves[from];
                    shuffle(&mut agent_in_mind_shelf.items, &mut rng);
                    state.shelf = agent_in_mind_shelf.clone();
                    // change to Predicting state
                    scene = Scene::Predicting;
                } else {
                    // shuffle the items or not depends on the random state
                    println!("Shuffling the deck...");
                    match rng.gen_bool(0.5) {
                        true => {
                            shuffle(&mut state.shelf.items, &mut rng);
                        }
                        _ => {}
                    }
                    // randomly change to one of the following states
                    // 1. DecisionMaking
                    // 2. Shuffling
                    match rng.gen_bool(0.5) {
                        true => {
                            scene = Scene::DecisionMaking;
                        }
                        false => {
                            scene = Scene::Shuffling;
                        }
                    }
                }
            }
            Scene::Peeping => {
                // agent is allowed to peep the status of the monitor depends on the random state (1) can peep, (0) can't peep
                let decision = state.agent_decision.unwrap();
                let agent_idx = match decision {
                    AgentDecision::Peep { from } => from,
                    _ => panic!("Invalid decision"),
                };
                let request_result: bool = rng.gen_bool(0.5);
                match request_result {
                    true => {
                        // agent can peep the status of the monitor
                        println!("Peeping the monitor...");
                        let agent_in_mind_shelf = &mut agent_inmind_shelves[agent_idx];
                        agent_in_mind_shelf.items = state.shelf.items.clone();
                    }
                    false => {
                        // agent can't peep the status of the monitor
                        println!("Can't peep the monitor...");
                    }
                }
                state.agent_decision = None;

                // randomly change to one of the following states
                //1. Shuffling
                //2. DecisionMaking
                match rng.gen_bool(0.5) {
                    true => {
                        scene = Scene::Shuffling;
                    }
                    false => {
                        scene = Scene::DecisionMaking;
                    }
                }
            }
            Scene::Predicting => {
                let agent_idx = match state.agent_decision.unwrap() {
                    AgentDecision::TakeItem { from } => from,
                    _ => panic!("Invalid decision"),
                };
                let real_item_idx = state.shelf.get_item_idx_by_belongs(agent_idx);
                // ask LLM to make prediction
                println!("Predicting the agent's action...");
                let predicted_inmind_item_idx: usize = rng.gen_range(0..agent_n); //TODO: LLM should predict the agent's action

                // move real item to predicted inmind item position
                state.shelf.exchange_items(real_item_idx, predicted_inmind_item_idx);

                // check if the agent get the correct item
                let item = state.shelf.items.remove(predicted_inmind_item_idx);
                if item.belongs_to == agent_idx {
                    println!("Agent {} get the correct item!", agent_idx);
                    state.score += 1;
                } else {
                    println!("Agent {} get the wrong item!", agent_idx);
                }
                agent_inmind_shelves.remove(agent_idx);

                //TODO: tell LLM current shelf status

                state.agent_decision = None;

                // randomly change to one of the following states
                //1. Shuffling
                //2. DecisionMaking 
                //3. End (if and only if there is no items left)
                if agent_inmind_shelves.is_empty() {
                    scene = Scene::End;
                } else {
                    match rng.gen_bool(0.5) {
                        true => {
                            scene = Scene::Shuffling;
                        }
                        false => {
                            scene = Scene::DecisionMaking;
                        }
                    }
                }
            }
            Scene::End => {
                // tell the final result, and game over
                println!("Correct: {}; Final score: {}", state.score, state.score * 100 / agent_n);
                println!("Game over!");
                break;
            }
        }
    }
}
