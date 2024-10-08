import { Component, OnDestroy, OnInit, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { InputNumberModule } from 'primeng/inputnumber';
import { FormsModule } from '@angular/forms';
import { SidebarService } from '../../services/sidebarService/sidebar.service';
import { SidebarComponent } from "../components/sidebar/sidebar.component";
import { ButtonModule } from 'primeng/button';
import { RippleModule } from 'primeng/ripple';
import { ActivatedRoute, Router } from '@angular/router';
import { RustService } from '../../services/rust/rust.service';
import { MidiSignal } from '../../model/MidiSignal';
import { MusicService } from '../../services/musicService/music.service';
import { MidiMusic } from '../../model/MidiMusic';
import { MidiState } from '../../model/MidiState';

import * as fw from 'three';

@Component({
  selector: 'app-gamificada',
  standalone: true,
  imports: [
    CommonModule, 
    FormsModule, 
    InputNumberModule, 
    SidebarComponent,
    ButtonModule,
    RippleModule
  ],
  templateUrl: './gamificada.component.html',
  styleUrl: './gamificada.component.scss'
})
export class GamificadaComponent implements OnInit, OnDestroy {

  public row: number = 0;

  private musicState: MidiState;
  private musicData: MidiMusic;

  constructor(
    protected sidebarService: SidebarService,
    private router: Router,
    private route: ActivatedRoute,
    private rust: RustService,
    private musicService: MusicService
  ) {
  }
  
  public ngOnInit(): void {
    const scene = new fw.Scene();
    const camera = new fw.PerspectiveCamera( 75, window.innerWidth/ window.innerHeight, 0.1, 1000);
    const renderer = new fw.WebGLRenderer();
    renderer.setSize( window.innerWidth, window.innerHeight );
    document.body.appendChild(renderer.domElement);

    const queryParam = this.route.snapshot.queryParamMap.get('id');
    if(!queryParam) this.router.navigate(['menu-gamificada']);
    const musicId = parseInt(queryParam!);
    this.rust.startMusic(musicId);
    this.musicData = this.musicService.getMusicById(musicId);

    this.rust.connectOcarina();
    this.rust.listenForOcarinaNote((note: MidiSignal) => {});

    this.rust.listenForMusicState((state: MidiState) => {
      this.musicState = state;
    });
    
  }

  public async ngOnDestroy(): Promise<void> {
    try {
      if(this.musicState != "PAUSED") await this.rust.stopMusic();
    } catch (error) { 
      console.log("Something went wrong, but the music is not playing..."); 
    }
    await this.rust.unlistenMidiNotes();
    this.rust.releaseOcarina();
  }

  public returnToGameMenu(): void {
    this.router.navigate(['menu-gamificada']);
  }

}