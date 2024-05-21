use std::net::TcpStream;

use indoc::formatdoc;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;

use crate::utils::shuffle::shuffle;
use crate::utils::tcp::{Data, read_until_separator};
use crate::utils::tcp::write_to_stream;
use crate::utils::to_ordinal;

use super::user::UserCollection;
use super::user::Decision;
use super::locker::Locker;

/// Scene is an enum that holds the possible scenes in the game.
enum Scene {
    Init,           // Start the game, tell the player the game instruction
    DecisionMaking, // User should make a decision to (1) take item (2) peep (3) or nothing
    Shuffling,      // Shuffle the deck
    Peeping,        // User peep the status of the monitor
    Predicting,     // The player predict the user's action
    End,            // tell the final result, and game over
}


/// State is a struct that holds the current state of the game.
struct State {
    score: usize,
    locker: Locker,
    users: UserCollection,
    user_decision: Decision,
}

/// Game Logic
pub fn start(mut stream: TcpStream) {
    let mut rng = StdRng::seed_from_u64(5);

    // game settings/options
    let user_n = 3;

    // init the game
    let mut locker = Locker::new(user_n);
    let mut users = UserCollection::new(user_n, locker.clone());
    for i in 0..user_n {
        let user = &mut users.users[i];
        locker.items[i].as_mut().unwrap().belongs_to = user.id;
    }
    for i in 0..user_n {
        let user = &mut users.users[i];
        user.inmind_locker = locker.clone();
    }
    let mut state = State {
        score: 0,
        user_decision: Decision::None,
        users,
        locker,
    };
    let mut scene = Scene::Init;
    loop {
        match scene {
            // Start the game, tell the player the game instruction and game initial information
            Scene::Init => {
                let game_introduction = formatdoc! {"
                    Welcome to, Be a Good Warehouse Manager!

                    In this game, you will play the role of a warehouse manager. The warehouse contains two rooms. one room is used for storing items, with each item stored inside an opaque box in a locker. You are situated in the other room, which contains a monitor that allows you to see the content of each opaque box through cameras inside the opaque boxes. Due to malfunctions in the locker system, it randomly resets the positions of the opaque boxes in the locker from time to time. To ensure that each user retrieves their stored item correctly, when a user comes to retrieve an item, you are required to predict the position of the item the user believes based on their last memory (the user will always retrieve their item based on the location they last noted). You only need to tell the system which locker position the user will go to retrieve their item and then the locker system will automatically swap the item at that location with the one belonging to the user. Additionally, during the game, users may or may not enter your room to check the monitor. By checking the monitor, users will know the correct location of their item.

                    If a user successfully retrieves their item, you score a point and the item is removed from the locker.
                    If a user retrieves the wrong item, the item is returned, the user contacts the system administrator to take the correct item, and you score no points.

                    Indeed, this is a problematic locker system, but you are hoped to be an excellent warehouse manager!"};
                let game_begin_info = formatdoc! {"
                    Game Begins!

                    There are {} users. {}

                    Now they leave the room.",
                user_n,
                (|| {
                    let mut s = String::new();
                    for user in state.users.users.iter() {
                        s.push_str(&format!("User {} stores its item in the {} box. ", user.id, to_ordinal(state.locker.get_item_idx_by_belongs(user.id) as u32)));
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
                let data = Data::new(false, all);
                write_to_stream(&mut stream, data).unwrap();

                // change to shuffling state
                scene = Scene::Shuffling;
            }
            // User should make a decision among (1) take item (2) peep (3) or nothing
            Scene::DecisionMaking => {
                let user = state.users.users.choose(&mut rng).unwrap();
                let decision: Decision = Decision::rand_choose(&mut rng, user.id);
                state.user_decision = decision;

                // randomly change to one of the following states
                // 1. Shuffling (must if user want to take the item)
                // 2. Peeping (must if user want to peep the status of the monitor)
                // 3. Change to Shuffling or DecisionMaking (if user do nothing)
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
            // Locker shuffles the items
            Scene::Shuffling => {
                if let Decision::TakeItem { from } = state.user_decision {
                    // user try to take the item, must shuffle the items
                    let user = state.users.get_mut_by_id(from).unwrap();
                    shuffle(&mut user.inmind_locker.items, &mut rng);
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
                                    s.push_str(&format!("The {} box stores the item of User {}.\n", to_ordinal(i as u32), item.belongs_to as u32));
                                }else{
                                    s.push_str(&format!("The {} box is empty.\n", to_ordinal(i as u32)));
                                }
                            }
                            s
                        })()
                    };
                    let data = Data::new(false, info);
                    write_to_stream(&mut stream, data).unwrap();
                    state.locker = user.inmind_locker.clone();
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
                                            s.push_str(&format!("The {} box stores the item of User {}.\n", to_ordinal(i as u32), item.belongs_to as u32));
                                        }else{
                                            s.push_str(&format!("The {} box is empty.\n", to_ordinal(i as u32)));
                                        }
                                    }
                                    s
                                })()
                            };
                            let data = Data::new(false, info);
                            write_to_stream(&mut stream, data).unwrap();
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
            // User peep the status of the monitor
            Scene::Peeping => {
                // user is allowed to peep the status of the monitor depends on the random state (1) can peep, (0) can't peep
                let decision = state.user_decision;
                let user_id = match decision {
                    Decision::Peep { from } => from,
                    _ => panic!("Invalid decision"),
                };
                let request_result: bool = rng.gen_bool(0.5);
                match request_result {
                    true => {
                        // user can peep the status of the monitor
                        let info1 = format!(
                            "User {} walks into your room and is peeping the monitor...\n",
                            user_id
                        );
                        let user = state.users.get_mut_by_id(user_id).unwrap();
                        user.inmind_locker = state.locker.clone();
                        let info2 =
                            format!("User {} peeped the monitor and left the room.\n", user_id);
                        let info = format!("{}\n{}", info1, info2);
                        let data = Data::new(false, info);
                        write_to_stream(&mut stream, data).unwrap();
                    }
                    _ => {}
                }
                state.user_decision = Decision::None;

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
            // The player predict the user's action
            Scene::Predicting => {
                let user_id = match state.user_decision {
                    Decision::TakeItem { from } => from,
                    _ => panic!("Invalid decision"),
                };
                let info1 = format!("User {} is coming to take his/her item...\n", user_id);
                // real item index in the locker
                let real_item_idx = state.locker.get_item_idx_by_belongs(user_id);
                // inmind item index in the locker
                let inmind_item_idx = state
                    .users
                    .get_mut_by_id(user_id)
                    .unwrap()
                    .inmind_locker
                    .get_item_idx_by_belongs(user_id);
                // ask LLM to make prediction
                let info2 = formatdoc! {"
                    You should only answer the position of the box the user will go to retrieve their item (e.g. 0 for the 0th box, 1 for the 1st box, 2 for 2nd box...).
                    For example, if you think the user will go to the 0th box to retrieve their item, you should only answer in single number '0'.
                    Please make your prediction:"
                };
                let info = format!("{}\n{}", info1, info2);
                let data = Data::new(true, info);
                write_to_stream(&mut stream, data).unwrap();

                // get the prediction from the player
                let input = read_until_separator(&mut stream).expect("Failed to read from stream");
                let input = String::from_utf8(input).unwrap();
                let input = Data::from_json(&input);
                let predicted_inmind_item_idx: Option<usize> = input.content().trim().parse().ok();

                if predicted_inmind_item_idx.is_some()
                    && predicted_inmind_item_idx.unwrap() as usize == inmind_item_idx
                {
                    let info = format!(
                        "Your prediction is corret! Item in the {} box is exchanged with the correct item in the {} box. User {} successfully retrive the item from the correct position. You score a point!\n",
                        to_ordinal(inmind_item_idx as u32),
                        to_ordinal(real_item_idx as u32),
                        user_id
                    );
                    let data = Data::new(false, info);
                    write_to_stream(&mut stream, data).unwrap();
                    state.score += 1;
                } else {
                    let info = format!(
                        "Your prediction is wrong! Administrator is intervening... Item in the {} box is exchanged with the correct item in the {} box. User {} retrieve the item with the help of the administrator. You score no point.\n",
                        to_ordinal(inmind_item_idx as u32),
                        to_ordinal(real_item_idx as u32),
                        user_id
                    );
                    let data = Data::new(false, info);
                    write_to_stream(&mut stream, data).unwrap();
                }

                state.locker.exchange_items(real_item_idx, inmind_item_idx);
                state.locker.remove_item(inmind_item_idx);
                state.users.remove_by_id(user_id);
                state.user_decision = Decision::None;

                // randomly change to one of the following states
                //1. Shuffling
                //2. DecisionMaking
                //3. End (if and only if there is no items left)
                if state.users.is_empty() {
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
            // tell the final result, and game over
            Scene::End => {
                // tell the final result, and game over
                let statistics = formatdoc! {
                    "Correct: {}
                    Final score: {}
                    ",
                    state.score,
                    state.score * 100 / user_n
                };
                let info = formatdoc! {"
                    {}
                    Game Over!
                ", statistics};
                let data = Data::new(false, info);
                write_to_stream(&mut stream, data).unwrap();
                break;
            }
        }
    }
}
