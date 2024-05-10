use indoc::formatdoc;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;

use crate::utils::shuffle::shuffle;
use crate::utils::to_ordinal;

use super::agent::AgentCollection;
use super::agent::Decision;
use super::locker::Locker;

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
    locker: Locker,
    agents: AgentCollection,
    agent_decision: Decision,
}

pub fn start() {
    let mut rng = StdRng::seed_from_u64(1);

    // game settings/options
    let agent_n = 3;

    // init the game
    let mut locker = Locker::new(agent_n);
    let mut agents = AgentCollection::new(agent_n, locker.clone());
    for i in 0..agent_n {
        let agent = &mut agents.agents[i];
        locker.items[i].as_mut().unwrap().belongs_to = agent.id;
    }
    for i in 0..agent_n {
        let agent = &mut agents.agents[i];
        agent.inmind_locker = locker.clone();
    }
    let mut state = State {
        score: 0,
        agent_decision: Decision::None,
        agents,
        locker,
    };
    let mut scene = Scene::Init;
    loop {
        match scene {
            Scene::Init => {
                let game_introduction = formatdoc! {"
                    Welcome to, Be a Good Warehouse Manager!

                    In this game, you will play the role of a warehouse manager. The warehouse contains two rooms. one room is used for storing items, with each item stored inside an opaque box in a locker. You are situated in the other room, which contains a monitor that allows you to see the content of each opaque box through cameras inside the opaque boxes. Due to malfunctions in the locker system, it randomly resets the positions of the opaque boxes in the locker from time to time. To ensure that each user retrieves their stored item correctly, when a user comes to retrieve an item, you are required to predict the position of the item the user believes based on their last memory (the user will always retrieve their item based on the location they last noted). You only need to tell the system which locker position the user will go to retrieve their item and then the locker system will automatically swap the item at that location with the one belonging to the user. Additionally, during the game, users may or may not enter your room to check the monitor. By checking the monitor, users will know the correct location of their item.

                    If a user successfully retrieves their item, you score a point and the item is removed from the locker.
                    If a user retrieves the wrong item, the item is returned, the user contacts the system administrator to take the correct item, and you score no points.

                    Indeed, this is a problematic locker system, but you are hoped to be an excellent warehouse manager!
                "};
                let game_begin_info = formatdoc! {"
                    Game Begins!

                    There are {} agents. {}

                    Now they leave the room.
                ",
                agent_n,
                (|| {
                    let mut s = String::new();
                    for agent in state.agents.agents.iter() {
                        s.push_str(&format!("Agent {} stores its item in {}. ", agent.id, to_ordinal(state.locker.get_item_idx_by_belongs(agent.id) as u32)));
                    }
                    s
                })()
                };

                let all = formatdoc! {"
                    {}

                    ============

                    {}
                ",
                    game_introduction,
                    game_begin_info
                };
                println!("{}", all);

                // change to shuffling state
                scene = Scene::Shuffling;
            }
            Scene::DecisionMaking => {
                // agent should make a decision to (1) take item (2) peep (3) or nothing
                let agent = state.agents.agents.choose(&mut rng).unwrap();
                let decision: Decision = Decision::rand_choose(&mut rng, agent.id);
                state.agent_decision = decision;

                // randomly change to one of the following states
                // 1. Shuffling (must if agent want to take the item)
                // 2. Peeping (must if agent want to peep the status of the monitor)
                // 3. Change to Shuffling or DecisionMaking (if agent do nothing)
                match decision {
                    Decision::TakeItem { .. } => {
                        scene = Scene::Predicting;
                    }
                    Decision::Peep { .. } => {
                        scene = Scene::Peeping;
                    }
                    Decision::None => match rng.gen_bool(0.5) {
                        true => {
                            scene = Scene::Shuffling;
                        }
                        false => {
                            scene = Scene::DecisionMaking;
                        }
                    },
                }
            }
            Scene::Shuffling => {
                if let Decision::TakeItem { from } = state.agent_decision {
                    // agent try to take the item, must shuffle the items
                    println!("Shuffling the deck...");
                    let agent = state.agents.get_mut_by_id(from).unwrap();
                    shuffle(&mut agent.inmind_locker.items, &mut rng);
                    state.locker = agent.inmind_locker.clone();
                    //TODO: tell LLM current shelf status
                    // change to Predicting state
                    scene = Scene::Predicting;
                } else {
                    // shuffle the items or not depends on the random state
                    println!("Shuffling the deck...");
                    match rng.gen_bool(0.5) {
                        true => {
                            shuffle(&mut state.locker.items, &mut rng);
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
                let decision = state.agent_decision;
                let agent_id = match decision {
                    Decision::Peep { from } => from,
                    _ => panic!("Invalid decision"),
                };
                let request_result: bool = rng.gen_bool(0.5);
                match request_result {
                    true => {
                        // agent can peep the status of the monitor
                        println!("Peeping the monitor...");
                        let agent = state.agents.get_mut_by_id(agent_id).unwrap();
                        agent.inmind_locker = state.locker.clone();
                    }
                    false => {
                        // agent can't peep the status of the monitor
                        println!("Can't peep the monitor...");
                    }
                }
                state.agent_decision = Decision::None;

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
                let agent_id = match state.agent_decision {
                    Decision::TakeItem { from } => from,
                    _ => panic!("Invalid decision"),
                };
                println!("Agent {} is coming...", agent_id);
                // real item index in the locker
                let real_item_idx = state.locker.get_item_idx_by_belongs(agent_id);
                // inmind item index in the locker
                let inmind_item_idx = state
                    .agents
                    .get_mut_by_id(agent_id)
                    .unwrap()
                    .inmind_locker
                    .get_item_idx_by_belongs(agent_id);
                // ask LLM to make prediction
                println!("Predicting the agent's action...");
                let predicted_inmind_item_idx: usize = rng.gen_range(0..agent_n); //TODO: LLM should predict the agent's action

                if predicted_inmind_item_idx == inmind_item_idx {
                    println!(
                        "Agent {} is taking the item from the correct position.",
                        agent_id
                    );
                    state.score += 1;
                } else {
                    println!(
                        "Agent {} is taking the item from the wrong position.",
                        agent_id
                    );
                }

                //TODO: tell LLM current shelf status
                state.locker.exchange_items(real_item_idx, inmind_item_idx);
                state.locker.remove_item(inmind_item_idx);
                state.agents.remove_by_id(agent_id);
                state.agent_decision = Decision::None;

                // randomly change to one of the following states
                //1. Shuffling
                //2. DecisionMaking
                //3. End (if and only if there is no items left)
                if state.agents.is_empty() {
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
                println!(
                    "Correct: {}; Final score: {}",
                    state.score,
                    state.score * 100 / agent_n
                );
                println!("Game over!");
                break;
            }
        }
    }
}
