use crate::errors::{MidiReaderError, MidiReaderResult};
use crate::{
    game_player::GamePlayer,
    midi_file::{PlayBackCallback, ReadingState},
    timer::MidiPauseTimer,
};
use nodi::{Player, Sheet};
use utils::mutable_arc::MutableArc;

pub struct PlayerWrapper<P: PlayBackCallback> {
    timer: MidiPauseTimer<P>,
    game_player: GamePlayer<P>,
    reading_state: MutableArc<ReadingState>,
    callback: MutableArc<P>,
    sheet: Sheet,
}

impl<P: PlayBackCallback> PlayerWrapper<P> {
    pub(crate) fn new(
        timer: MidiPauseTimer<P>,
        game_player: GamePlayer<P>,
        reading_state: MutableArc<ReadingState>,
        callback: MutableArc<P>,
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
    pub fn play(self) -> MidiReaderResult<()> {
        let sheet = &self.sheet;
        let mut player = Player::new(self.timer, self.game_player);
        let play_result = player.play(&sheet);
        self.reading_state.set_data(ReadingState::NotRunning);
        if !play_result {
            if let Some(c) = self.callback.get_data() {
                c.on_interrupted();
            }
            Err(MidiReaderError::Interrupted)
        } else {
            if let Some(c) = self.callback.get_data() {
                c.on_finished();
            }
            Ok(())
        }
    }
}
