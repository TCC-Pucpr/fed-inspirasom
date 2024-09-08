import {Injectable} from '@angular/core';
import {MidiMusic} from '../../model/MidiMusic';
import {RustService} from '../rust/rust.service';

@Injectable({
    providedIn: 'root'
})
export class MusicService {

    private musicList: MidiMusic[] = [];

    constructor(
        private rust: RustService,
    ) {
    }


    public async fetchMusicList(): Promise<MidiMusic[]> {
        this.musicList = (await this.rust.getMusicList()).files;
        return this.musicList;
    }

    public setMusicList(list: MidiMusic[]): void {
        this.musicList = list;
    }

    public getMusicList(): MidiMusic[] {
        return this.musicList;
    }

    public getMusicById(id: number): MidiMusic {
        const filter = this.musicList.filter(music => music.id === id)[0];
        return filter ? filter : {} as MidiMusic;
    }

}
