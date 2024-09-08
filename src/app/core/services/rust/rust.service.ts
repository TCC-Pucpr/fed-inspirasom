import {Injectable} from '@angular/core';
import {invoke} from '@tauri-apps/api/tauri';
import {RustEventsName, RustFunctionName} from './rust-functions.enum';
import {MidiSignal} from "../../model/MidiSignal";
import {listen} from "@tauri-apps/api/event";
import {MidiMusicList} from '../../model/MidiMusicList';

@Injectable({
    providedIn: 'root'
})
export class RustService {

    private listeningMidiNotes: any;

    constructor() {
    }

    public connect_midi() {
        invoke(RustFunctionName.connectMidi).then(_ => {
        });
    }

    public stop_midi() {
        invoke(RustFunctionName.stopMidi).then(_ => {
        });
    }

    public listen_for_midi_note(callback: (signal: MidiSignal) => void) {
        return listen(RustEventsName.midiNote, (event) => {
            callback(event.payload as MidiSignal)
        });
    }

    public async getMusicList(): Promise<MidiMusicList> {
        return await invoke(RustFunctionName.listMusics);
    }

    public async startMusic(musicId: number): Promise<void> {
        await invoke(RustFunctionName.startGame, {musicId}).then(_ => {
        });
    }

    public async pauseMusic() {
        await invoke(RustFunctionName.pauseGame);
    }

    public async resumeMusic() {
        await invoke(RustFunctionName.resumeGame);
    }

    public async stopMusic(): Promise<void> {
        await invoke(RustFunctionName.stopgame).then(_ => {
        });
    }

    public async listenMidiNotes(callback: (signal: MidiSignal) => void) {
        this.listeningMidiNotes = listen(RustEventsName.midiReadNote, (event) => {
            callback(event.payload as MidiSignal);
        });
        return this.listeningMidiNotes;
    }

    public async unlistenMidiNotes() {
        this.listeningMidiNotes.then((_: any) => {
        });
    }
}
