import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';
import {RustEventsName, RustFunctionName} from '../rust-functions.enum';
import {MidiSignal} from "../../../model/MidiSignal";
import {listen} from "@tauri-apps/api/event";

@Injectable({
  providedIn: 'root'
})
export class RustDataSourceService {

  constructor() { }

   /**
    * Função exemplo de comunicação entre rust e angular
    * @param event Evento teste
    * @param message mensagem que irá para o rust
    * @returns string com uma mensagem montada
    */
   public async greet(event: SubmitEvent, message: string): Promise<string> {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    return await invoke<string>(RustFunctionName.greet, { name: message } );
  }

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
}
