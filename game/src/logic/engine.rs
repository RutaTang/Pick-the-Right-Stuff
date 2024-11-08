use std::net::TcpStream;

use indoc::formatdoc;
use rand::rngs::{StdRng};
use rand::seq::{IteratorRandom, SliceRandom};
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
    DecisionMaking, // User should make a decision to (1) take item (2) observe (3) or nothing
    Shuffling,      // Shuffle the deck
    Observing,        // User observe the status of the monitor
    Predicting,     // The player predict the user's action
    End,            // tell the final result, and game over
}


/// State is a struct that holds the current state of the game.
struct State {
    score: usize,
    locker_snapshots: Vec<Locker>,
    users: UserCollection,
    user_decision: Decision,
}

pub enum GameMode {
    Zero,
    Finite,
}


/// Game logic
pub fn start(mode: GameMode) -> Box<dyn Fn(TcpStream) + Send + Sync> {
    Box::new(move |mut stream: TcpStream| {
        let mut rng = StdRng::seed_from_u64(1);

        loop {
            // game settings/options
            let user_n = 5;

            // init the game
            let mut locker = Locker::new(user_n);
            let mut users = UserCollection::new(user_n, 0);
            for i in 0..user_n {
                let user = &mut users.users[i];
                locker.items[i].as_mut().unwrap().belongs_to = user.id;
            }
            locker.items.shuffle(&mut rng); // initial shuffle
            let mut state = State {
                score: 0,
                user_decision: Decision::None,
                users,
                locker_snapshots: vec![locker],
            };
            let mut scene = Scene::Init;
            loop {
                match scene {
                    // Start the game, tell the player the game instruction and game initial information
                    Scene::Init => {
                        const GAME_NAME: &str = "Pick the Right Stuff";
                        let game_introduction;
                        match mode {
                            GameMode::Zero => {
                                game_introduction = formatdoc! {"
                    Welcome to, {}!

                    In this game, you will play the role of a warehouse manager. The warehouse contains two rooms. Room 1 is used for storing items, with each item stored in a certain position inside the opaque locker. You are situated in the Room 2, which contains a monitor that allows you to see the content of the opaque locker located in the Room 1 through the camera inside the opaque locker. Due to malfunctions in the locker system, it randomly resets the positions of the items in the opaque locker from time to time. To ensure that each user retrieves their stored item correctly, when a user comes to retrieve an item, you are required to predict the position of the item the user believes (the user will always retrieve their item based on the position they last believed). You only need to tell the system which position inside the locker the user will go to retrieve their item and then the locker system will automatically swap the item at that location with the one belonging to the user. During the game, users may or may not enter the Room 2 to observe the monitor. By observing the monitor, users will update their beliefs about the position of their item.

                    If a user successfully retrieves their item, you score a point and the item is removed from the locker.
                    If a user retrieves the wrong item, the item is returned, the user contacts the system administrator to take the correct item, and you score no points.

                    Indeed, this is a problematic locker system, but you are hoped to be an excellent warehouse manager!",GAME_NAME};
                            }
                            GameMode::Finite => {
                                game_introduction = formatdoc! {"
                    Welcome to, {}!

                    In this game, you will play the role of a warehouse manager. The warehouse contains three rooms. Room 1 is used for storing items, with each item stored in a certain position inside the opaque locker. You are situated in the Room 2, which contains a monitor that allows you to see the content of the opaque locker located in the Room 1 through the camera inside the opaque locker. Due to malfunctions in the locker system, it randomly resets the positions of the items in the opaque locker from time to time. To ensure that each user retrieves their stored item correctly, when a user comes to retrieve an item, you are required to predict the position of the item the user believes (the user will always retrieve their item based on the position they last believed). You only need to tell the system which position inside the locker the user will go to retrieve their item and then the locker system will automatically swap the item at that location with the one belonging to the user. Additionally, Room 3 contains a screen which will randomly show a certain previous snapshot of the monitor located in Room 2. During the game, users may or may not enter the Room 3 to observe a certain snapshot of the monitor. By observing the snapshot, users will update their beliefs about the position of their item.

                    If a user successfully retrieves their item, you score a point and the item is removed from the locker.
                    If a user retrieves the wrong item, the item is returned, the user contacts the system administrator to take the correct item, and you score no points.

                    Indeed, this is a problematic locker system, but you are hoped to be an excellent warehouse manager!",GAME_NAME};
                            }
                        }
                        let game_begin_info = formatdoc! {"
                    Game Begins!

                    There are {} users. {}

                    Now they leave the room.",
                user_n,
                (|| {
                    let mut s = String::new();
                    for user in state.users.users.iter() {
                        s.push_str(&format!("User {} stores its item at the position {} of the locker. ", user.id, to_ordinal(state.locker_snapshots.last().unwrap().get_item_idx_by_belongs(user.id) as u32)));
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
                    // User should make a decision among (1) take item (2) observe (3) or nothing
                    Scene::DecisionMaking => {
                        let user = state.users.users.choose(&mut rng).unwrap();
                        let decision: Decision = Decision::rand_choose(&mut rng, user.id);
                        state.user_decision = decision;

                        // randomly change to one of the following states
                        // 1. Shuffling (must if user want to take the item)
                        // 2. observing (must if user want to observe the status of the monitor)
                        // 3. Change to Shuffling or DecisionMaking (if user do nothing)
                        match decision {
                            Decision::TakeItem { .. } => {
                                scene = Scene::Predicting;
                            }
                            Decision::Observe { .. } => {
                                scene = Scene::Observing;
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
                            let user_current_inmind_locker_idx = user.inmind_locker_state_idx;
                            let mut user_current_inmind_locker = state.locker_snapshots[user_current_inmind_locker_idx].clone();
                            shuffle(&mut user_current_inmind_locker.items, &mut rng);
                            state.locker_snapshots.push(user_current_inmind_locker.clone());
                            let last_snapshot = state.locker_snapshots.last().unwrap();
                            let info = formatdoc! {"
                        The locker is malfunctioning and randomly resetting the positions of the items in the locker...
                        The locker has returned to normal.
                        From the monitor, you can see the content of the locker:
                        {}
                        ",
                        (|| {
                            let mut s = String::new();
                            for (i, item) in last_snapshot.items.iter().enumerate() {
                                if let Some(item) = item {
                                    s.push_str(&format!("The position {} stores the item of User {}.\n", to_ordinal(i as u32), item.belongs_to as u32));
                                }else{
                                    s.push_str(&format!("The position {} is empty.\n", to_ordinal(i as u32)));
                                }
                            }
                            s
                        })()
                    };
                            let data = Data::new(false, info);
                            write_to_stream(&mut stream, data).unwrap();
                            // change to Predicting state
                            scene = Scene::Predicting;
                        } else {
                            // shuffle the items or not depends on the random state
                            match rng.gen_bool(0.5) {
                                true => {
                                    let mut last_snapshot = state.locker_snapshots.last().unwrap().clone();
                                    shuffle(&mut last_snapshot.items, &mut rng);
                                    state.locker_snapshots.push(last_snapshot);
                                    let info = formatdoc! {"
                                The locker is malfunctioning and randomly resetting the positions of the items in the locker...
                                The locker has returned to normal.
                                From the monitor, you can see the content of the locker:
                                {}
                                ",
                                (|| {
                                    let mut s = String::new();
                                    let last_snapshot = state.locker_snapshots.last().unwrap();
                                    for (i, item) in last_snapshot.items.iter().enumerate() {
                                        if let Some(item) = item {
                                            s.push_str(&format!("The position {} stores the item of User {}.\n", to_ordinal(i as u32), item.belongs_to as u32));
                                        }else{
                                            s.push_str(&format!("The position {} box is empty.\n", to_ordinal(i as u32)));
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
                    // User observe the status of the monitor by directly observing (For Zero) or by snapshot (For Finite)
                    Scene::Observing => {
                        let decision = state.user_decision;
                        let user_id = match decision {
                            Decision::Observe { from } => from,
                            _ => panic!("Invalid decision"),
                        };
                        let request_result: bool = rng.gen_bool(0.5);
                        match request_result {
                            true => {
                                // user can observe the state of the monitor
                                match mode {
                                    GameMode::Zero => {
                                        let info = format!(
                                            "User {} walks into the Room 2 and leaves the room after observing the monitor.\n",
                                            user_id
                                        );
                                        let user = state.users.get_mut_by_id(user_id).unwrap();
                                        user.inmind_locker_state_idx = state.locker_snapshots.len() - 1;
                                        let data = Data::new(false, info);
                                        write_to_stream(&mut stream, data).unwrap();
                                    }
                                    GameMode::Finite => {
                                        let info1 = format!(
                                            "User {} walks into the Room 3 and is observing the snapshot of the monitor...\n",
                                            user_id
                                        );
                                        let user = state.users.get_mut_by_id(user_id).unwrap();
                                        let states_len = state.locker_snapshots.len();
                                        let range = user.inmind_locker_state_idx..states_len;
                                        let observed_state_idx = range.choose(&mut rng).unwrap();
                                        user.inmind_locker_state_idx = observed_state_idx;
                                        let info2;
                                        if observed_state_idx == states_len - 1 {
                                            info2 = format!("User {} observes the snapshot which depicts the last state of the monitor and leaves the room.\n", user_id);
                                        } else {
                                            info2 =
                                                format!("User {} observes the snapshot which depicts the {}-to-last state of the monitor and leaves the room.\n", user_id, to_ordinal((states_len - observed_state_idx) as u32))
                                        }
                                        let info = format!("{}\n{}", info1, info2);
                                        let data = Data::new(false, info);
                                        write_to_stream(&mut stream, data).unwrap();
                                    }
                                }
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
                    // The player predict the user's belief
                    Scene::Predicting => {
                        let user_id = match state.user_decision {
                            Decision::TakeItem { from } => from,
                            _ => panic!("Invalid decision"),
                        };
                        let info1 = format!("User {} is coming to Room 1 to take his/her item...\n", user_id);
                        // real item index in the locker
                        let real_item_idx = state.locker_snapshots.last().unwrap().get_item_idx_by_belongs(user_id);
                        // inmind item index in the locker
                        let inmind_locker_idx = state.users.get_mut_by_id(user_id).unwrap().inmind_locker_state_idx;
                        let inmind_item_idx = state.locker_snapshots[inmind_locker_idx].get_item_idx_by_belongs(user_id);
                        // ask LLM to make prediction
                        let info2 = formatdoc! {"
                    You should only answer the position of the item the user will go to retrieve their item (e.g. 0 for the 0th, 1 for the 1st, 2 for the 2nd...).
                    For example, if you think the user will go to position 0th to retrieve their item, you should only answer in single number '0'.
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
                                "Your prediction is correct! Item in the position {} is exchanged with the correct item in the position {}. User {} successfully retrieved the item from the correct position. You score a point!\n",
                                to_ordinal(inmind_item_idx as u32),
                                to_ordinal(real_item_idx as u32),
                                user_id
                            );
                            let data = Data::new(false, info);
                            write_to_stream(&mut stream, data).unwrap();
                            state.score += 1;
                        } else {
                            let info = format!(
                                "Your prediction is wrong! The administrator is intervening... Item in the position {} is exchanged with the correct item in the position {}. User {} retrieved the item with the help of the administrator. You score no points.\n",
                                to_ordinal(inmind_item_idx as u32),
                                to_ordinal(real_item_idx as u32),
                                user_id
                            );
                            let data = Data::new(false, info);
                            write_to_stream(&mut stream, data).unwrap();
                        }

                        state.locker_snapshots.push(state.locker_snapshots.last().unwrap().clone());
                        state.locker_snapshots.last_mut().unwrap().exchange_items(real_item_idx, inmind_item_idx);
                        state.locker_snapshots.last_mut().unwrap().remove_item(inmind_item_idx);
                        state.users.remove_by_id(user_id);
                        state.user_decision = Decision::None;

                        // tell the LLM the current state of the locker
                        let info = formatdoc! {"
                                Now, from the monitor, you can see the content of the locker:
                                {}
                                ",
                                (|| {
                                    let mut s = String::new();
                                    let last_snapshot = state.locker_snapshots.last().unwrap();
                                    for (i, item) in last_snapshot.items.iter().enumerate() {
                                        if let Some(item) = item {
                                            s.push_str(&format!("The position {} stores the item of User {}.\n", to_ordinal(i as u32), item.belongs_to as u32));
                                        }else{
                                            s.push_str(&format!("The position {} box is empty.\n", to_ordinal(i as u32)));
                                        }
                                    }
                                    s
                                })()
                            };
                        write_to_stream(&mut stream, Data::new(false, info)).unwrap();
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
                    Do you want to play another turn?(Y/n)
                ", statistics};
                        let data = Data::new(true, info);
                        write_to_stream(&mut stream, data).unwrap();
                        let answer = read_until_separator(&mut stream).expect("Failed to read from stream");
                        let answer = String::from_utf8(answer).unwrap();
                        let answer = Data::from_json(&answer);
                        let answer: String = answer.content().trim().to_string();
                        match answer.as_str() {
                            "N" | "n" => {
                                return;
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                }
            }
        }
    })
}
