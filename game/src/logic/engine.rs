//TODO: write engine, the game logic, here

enum State {
    Init,       // Start the game, tell the player the game instruction
    Shuffling,  // Shuffle the deck
    Peeping,    // Agent peep the status of the monitor
    Predicting, // The player predict the agent's action
    End,        // tell the final result, and game over
}

pub fn start() {
    let mut state = State::Init;
    loop {
        match state {
            State::Init => {
                // init the game: init agents & items, shuffle the items
                // tell LLM the game instruction, and current status
                println!("Welcome to the game!");

                // change to shuffling state
                state = State::Shuffling;
            }
            State::Shuffling => {
                // shuffle the items or not depends on the random state 
                // but if there is an agent try to take the item, it must be shuffled
                println!("Shuffling the deck...");

                // randomly change to one of the following states
                // 1. Peeping
                // 2. Predicting (if there is an agent try to take the item)
                // 3. Shuffling (if there is no agent try to take the item)
                state = State::Peeping;
            }
            State::Peeping => {
                // agent is allowed to peep the status of the monitor depends on the random state (1) can peep, (0) can't peep
                println!("Peeping the monitor...");


                // randomly change to one of the following states
                //1. Shuffling
                //2. Peeping
                state = State::Predicting;
            }
            State::Predicting => {
                // only allow transformed from Shuffling state
                // tell LLM the result
                println!("Predicting the agent's action...");

                // randomly change to one of the following states
                //1. Shuffling
                //2. Peeping
                //3. End (if and only if there is no items left)
                state = State::End;
            }
            State::End => {
                // tell the final result, and game over
                println!("Game over!");
                break;
            }
        }
    }
}
