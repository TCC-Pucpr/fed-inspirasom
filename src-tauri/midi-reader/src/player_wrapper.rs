use nodi::{Player, Sheet};

use crate::{
    errors::Interrupted,
    game_connection::GamePlayer,
    midi_file::{PlayBackCallback, ReadingState},
    timer::MidiPauserTimer,
    ArcMutex, Result,
};

pub struct PlayerWrapper<P: PlayBackCallback> {
    timer: MidiPauserTimer<P>,
    game_player: GamePlayer<P>,
    reading_state: ArcMutex<ReadingState>,
    callback: ArcMutex<P>,
    sheet: Sheet,
}

impl<P: PlayBackCallback> PlayerWrapper<P> {
    pub(crate) fn new(
        timer: MidiPauserTimer<P>,
        game_player: GamePlayer<P>,
        reading_state: ArcMutex<ReadingState>,
        callback: ArcMutex<P>,
        sheet: Sheet,
    ) -> Self {
        Self {
            timer,
            game_player,
            sheet,
            reading_state,
            callback,
        }
    }
    pub fn play(self) -> Result<()> {
        let sheet = &self.sheet;
        let mut player = Player::new(self.timer, self.game_player);
        let play_result = player.play(&sheet);
        if let Ok(mut m) = self.reading_state.lock() {
            *m = ReadingState::NotRunning;
        }
        if !play_result {
            if let Ok(c) = self.callback.lock() {
                c.on_interrupted();
            }
            Err(Interrupted.into())
        } else {
            if let Ok(c) = self.callback.lock() {
                c.on_finished();
            }
            Ok(())
        }
    }
}
