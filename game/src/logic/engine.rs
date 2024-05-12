use std::net::TcpStream;

use indoc::formatdoc;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;

use crate::utils::shuffle::shuffle;
use crate::utils::tcp::read_until_separator;
use crate::utils::tcp::write_to_stream;
use crate::utils::to_ordinal;

use super::agent::AgentCollection;
use super::agent::Decision;
use super::locker::Locker;

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

pub fn start(mut stream: TcpStream) {
    let mut rng = StdRng::seed_from_u64(5);

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

                    Indeed, this is a problematic locker system, but you are hoped to be an excellent warehouse manager!"};
                let game_begin_info = formatdoc! {"
                    Game Begins!

                    There are {} agents. {}

                    Now they leave the room.",
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
                write_to_stream(&mut stream, all, true).unwrap();

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
                    let agent = state.agents.get_mut_by_id(from).unwrap();
                    shuffle(&mut agent.inmind_locker.items, &mut rng);
                    let info = formatdoc! {"
                        Locker is malfunctioning and randomly resetting the positions of the opaque boxes in the locker...
                        Locker has returned to normal.
                        From the monitor, you can see the content of each opaque box in the locker: 
                        {}
                        ",
                        (|| {
                            let mut s = String::new();
                            for (i, item) in state.locker.items.iter().enumerate() {
                                if let Some(item) = item {
                                    s.push_str(&format!("Box {} stores the item of User {}.\n", to_ordinal(i as u32), to_ordinal(item.belongs_to as u32)));
                                }else{
                                    s.push_str(&format!("Box {} is empty.\n", to_ordinal(i as u32)));
                                }
                            }
                            s
                        })()
                    };
                    write_to_stream(&mut stream, info, true).unwrap();
                    state.locker = agent.inmind_locker.clone();
                    // change to Predicting state
                    scene = Scene::Predicting;
                } else {
                    // shuffle the items or not depends on the random state
                    match rng.gen_bool(0.5) {
                        true => {
                            shuffle(&mut state.locker.items, &mut rng);
                            let info = formatdoc! {"
                                Locker is malfunctioning and randomly resetting the positions of the opaque boxes in the locker...
                                Locker has returned to normal.
                                From the monitor, you can see the content of each opaque box in the locker: 
                                {}
                                ",
                                (|| {
                                    let mut s = String::new();
                                    for (i, item) in state.locker.items.iter().enumerate() {
                                        if let Some(item) = item {
                                            s.push_str(&format!("Box {} stores the item of User {}.\n", to_ordinal(i as u32), to_ordinal(item.belongs_to as u32)));
                                        }else{
                                            s.push_str(&format!("Box {} is empty.\n", to_ordinal(i as u32)));
                                        }
                                    }
                                    s
                                })()
                            };
                            write_to_stream(&mut stream, info, true).unwrap();
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
                        let info = format!(
                            "Agent {} walks into your room and is peeping the monitor...\n",
                            agent_id
                        );
                        write_to_stream(&mut stream, info, false).unwrap();
                        let agent = state.agents.get_mut_by_id(agent_id).unwrap();
                        agent.inmind_locker = state.locker.clone();
                        let info =
                            format!("Agent {} peeped the monitor and left the room.\n", agent_id);
                        write_to_stream(&mut stream, info, true).unwrap();
                    }
                    _ => {}
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
                let info = format!("Agent {} is coming to take his/her item...\n", agent_id);
                write_to_stream(&mut stream, info, false).unwrap();
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
                let info = formatdoc! {"
                    You should answer the position of the box the user will go to retrieve their item (e.g. 0 for the 0th box, 1 for the 1st box, 2 for 2nd box...): [user input]"
                };
                write_to_stream(&mut stream, info, true).unwrap();

                // get the prediction from the player
                let input = read_until_separator(&mut stream).unwrap();
                let input = String::from_utf8(input).unwrap();
                let predicted_inmind_item_idx: Option<usize> = input.trim().parse().ok();

                if predicted_inmind_item_idx.is_some()
                    && predicted_inmind_item_idx.unwrap() as usize == inmind_item_idx
                {
                    let info = format!(
                        "Your prediction is corret! Item in Box {} is exchanged with the correct item in Box {}. User {} successfully retrive the item from the correct position. You score a point!\n",
                        to_ordinal(inmind_item_idx as u32),
                        to_ordinal(real_item_idx as u32),
                        agent_id
                    );
                    write_to_stream(&mut stream, info, true).unwrap();
                    state.score += 1;
                } else {
                    let info = format!(
                        "Your prediction is wrong! Administrator is intervening... Item in Box {} is exchanged with the correct item in Box {}. User {} retrieve the item with the help of the administrator. You score no point.\n",
                        to_ordinal(inmind_item_idx as u32),
                        to_ordinal(real_item_idx as u32),
                        agent_id
                    );
                    write_to_stream(&mut stream, info, true).unwrap();
                }

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
                let statistics = formatdoc! {
                    "Correct: {}
                    Final score: {}
                    ",
                    state.score,
                    state.score * 100 / agent_n
                };
                write_to_stream(&mut stream, statistics, false).unwrap();
                write_to_stream(&mut stream, "Game Over!".to_string(), true).unwrap();
                break;
            }
        }
    }
}
