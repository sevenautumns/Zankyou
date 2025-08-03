pub enum State {
    MainMenuState(MainMenuState),
    GameModeState(GameModeState),
}

impl Default for State {
    fn default() -> Self {
        State::MainMenuState(MainMenuState {})
    }
}

#[derive(Default)]
pub struct MainMenuState {}

#[derive(Default)]
pub struct GameModeState {}
