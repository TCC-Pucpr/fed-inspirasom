import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';
import { RustEventsName, RustFunctionName } from './rust-functions.enum';
import { MidiSignal } from "../../model/MidiSignal";
import { listen } from "@tauri-apps/api/event";
import { MidiMusicList } from '../../model/MidiMusicList';

@Injectable({
  providedIn: 'root'
})
export class RustService {

  constructor() { }

  public connect_midi() {
    invoke(RustFunctionName.connectMidi).then(_ => {});
  }

  public stop_midi() {
    invoke(RustFunctionName.stopMidi).then(_ => {});
  }

   public listen_for_midi_note(callback: (signal: MidiSignal) => void) {
    return listen(RustEventsName.midiNote, (event) => {
      callback(event.payload as MidiSignal)
    });
   }

   public async getMusicList(): Promise<MidiMusicList> {
    return await invoke(RustFunctionName.listMusics);
   }

   public async startMusic(musicId: String): Promise<void> {
    await invoke(RustFunctionName.startGame, { musicId }).then(_ => {});
   }

   public async stopMusic(): Promise<void> {
    await invoke(RustFunctionName.stopgame).then(_ => {});
   }

   public async getMidiNotes(callback: (signal: any) => void) {
    return listen(RustEventsName.midiReadNote, (event) => {
      console.log(event);
      callback(event as any)
    });
   }
}
