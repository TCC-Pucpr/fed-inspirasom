import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';
import { RustEventsName, RustFunctionName } from './rust-functions.enum';
import { MidiSignal } from "../../model/MidiSignal";
import { listen } from "@tauri-apps/api/event";
import { MidiMusicList } from '../../model/MidiMusicList';
import { MidiState } from '../../model/MidiState';
import { OnNoteMessage } from '../../model/OnNoteMessage';

@Injectable({
    providedIn: 'root'
})
export class RustService {

  private listeningMidiNotes: any;
  private listeningMusicState: any;

    constructor() {
    }

    public connectOcarina() {
      invoke(RustFunctionName.connectMidi).then(_ => {});
    }

    public releaseOcarina() {
      invoke(RustFunctionName.stopMidi).then(_ => {});
    }

  public listenForOcarinaNote(callback: (signal: MidiSignal) => void) {
    return listen(RustEventsName.midiNote, (event) => {
      callback(event.payload as MidiSignal)
    });
  }

  public async getMusicList(): Promise<MidiMusicList> {
    return await invoke(RustFunctionName.listMusics);
  }

  public async startMusic(musicId: number): Promise<void> {
    await invoke(RustFunctionName.startGame, { musicId }).then(_ => {});
  }

  public async pauseMusic() {
    await invoke(RustFunctionName.pauseGame);
  }

  public async resumeMusic() {
    await invoke(RustFunctionName.resumeGame);
  }

  public async stopMusic(): Promise<void> {
    await invoke(RustFunctionName.stopgame).then(_ => {});
  }

  public async listenMidiNotes(callback: (signal: MidiSignal) => void) {
    this.listeningMidiNotes = listen(RustEventsName.midiReadNote, (event) => {
      callback(event.payload as MidiSignal);
    });
    return this.listeningMidiNotes;
  }

  public async unlistenMidiNotes() {
    this.listeningMidiNotes.then((_: any) => { });
  }

  public async listenForMusicState(callback: (state: MidiState) => void) {
    this.listeningMusicState = listen(RustEventsName.midiReadState, (event) => {
      callback(event.payload as MidiState);
    });
    return this.listeningMusicState;
  }

  public async unlistenMusicState() {
    this.listeningMusicState.then((_: any) => { });
  }

  public async endGameRust() {
    await invoke(RustFunctionName.endGame);
  }

  public async onInteractNote(data: OnNoteMessage){
    await invoke(RustFunctionName.onNote, { on_note_message: data });
  }
}
