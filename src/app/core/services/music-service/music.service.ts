import { Injectable } from '@angular/core';
import { MusicSheet } from '../../model/MusicSheet.model';
import { Note } from '../../model/Note';

@Injectable({
  providedIn: 'root'
})
export class MusicService {

  public musics: MusicSheet[];

  constructor() {
  }

  ngOnInit(): void {}

  public static mapTonality(note: Note): number {
    let offset = 0; 
    if(note.includes('b')){
      offset = 20;
    }
    return 203 + offset;
  }

}
