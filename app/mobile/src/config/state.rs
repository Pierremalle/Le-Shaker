use dioxus::prelude::*;

/// List of player across the game.
pub static PLAYERS: GlobalSignal<Vec<String>> = Signal::global(|| vec![]);
/// Current player index
pub static PLAYER_INDEX: GlobalSignal<usize> = Signal::global(|| 0);
/// Last player index, used to don't have a player two times in a row
pub static LAST_PLAYER: GlobalSignal<Option<String>> = Signal::global(|| None);

/// Used to get a new player randomly
pub fn next_player() -> Option<String> {
    let mut players = PLAYERS.write();
    let mut index = PLAYER_INDEX.write();
    let mut last = LAST_PLAYER.write();

    if players.is_empty() {
        return None;
    }

    // shuffle au début
    if *index == 0 {
        use rand::seq::SliceRandom;
        players.shuffle(&mut rand::rng());
    }

    let mut player = players.get(*index).cloned();

    // éviter doublon immédiat
    if let Some(ref last_player) = *last {
        if let Some(ref p) = player {
            if p == last_player && players.len() > 1 {
                *index = (*index + 1) % players.len();
                player = players.get(*index).cloned();
            }
        }
    }

    *index += 1;

    if *index >= players.len() {
        *index = 0;
    }

    *last = player.clone();

    player
}
