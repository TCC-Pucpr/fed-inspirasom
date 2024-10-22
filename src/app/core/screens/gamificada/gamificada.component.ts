import { Component, OnDestroy, OnInit, ViewChild } from '@angular/core';
import { GameComponent } from "./game/game.component";
import { CommonModule } from '@angular/common';
import { GameScene } from './game/scenes/Game.scene';
import { EventBus } from './game/events/EventBus';
import { EventNames } from './game/events/EventNames.enum';
import { InputNumberModule } from 'primeng/inputnumber';
import { FormsModule } from '@angular/forms';
import { SidebarService } from '../../services/sidebarService/sidebar.service';
import { SidebarComponent } from "../components/sidebar/sidebar.component";
import { ButtonModule } from 'primeng/button';
import { RippleModule } from 'primeng/ripple';
import { ActivatedRoute, Router } from '@angular/router';
import { RustService } from '../../services/rust/rust.service';
import { MidiSignal } from '../../model/MidiSignal';
import { PauseScene } from './game/scenes/Pause.scene';
import { MusicService } from '../../services/musicService/music.service';
import { MidiMusic } from '../../model/MidiMusic';
import { MidiState } from '../../model/MidiState';
import { EndgameScene } from './game/scenes/Endgame.scene';
import { OnNotePrecision } from '../../model/NotePressPrecision';
import { OnNoteMessage } from '../../model/OnNoteMessage';
import { NotePrecision } from '../../model/NotePrecision.model';

@Component({
  selector: 'app-gamificada',
  standalone: true,
  imports: [
    CommonModule, 
    GameComponent, 
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

  @ViewChild(GameComponent) phaserRef!: GameComponent;
  private gameScene: GameScene;
  private pauseScene: PauseScene;

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
    const queryParam = this.route.snapshot.queryParamMap.get('id');
    if(!queryParam) this.router.navigate(['menu-gamificada']);
    const musicId = parseInt(queryParam!);
    this.rust.startMusic(musicId);
    this.rust.listenMidiNotes(this.addNoteOnGame);
    this.musicData = this.musicService.getMusicById(musicId);

    this.rust.connectOcarina();
    this.rust.listenForOcarinaNote((note: MidiSignal) => {
      EventBus.emit(EventNames.ocarinaNote, note);
    });

    this.rust.listenForMusicState((state: MidiState) => {
      this.musicState = state;
      EventBus.emit(EventNames.musicStateChange, state);
    });
    
    EventBus.on(EventNames.gameSceneReady, (scene: GameScene) => {
      this.gameScene = scene;
    });

    EventBus.on(EventNames.pauseSceneReady, (scene: PauseScene) => {
      this.pauseScene = scene;
      this.pauseScene.musicName = this.musicData.name;
    });

    EventBus.on(EventNames.endSceneReady, (scene: EndgameScene) => {
      scene.musicName = this.musicData.name;
    });

    EventBus.on(EventNames.exitGame, (_: any) => {
      this.returnToGameMenu();
    });

    EventBus.on(EventNames.pauseGame, (_: any) => {
      if(this.musicState != "PAUSED") {
        this.rust.pauseMusic();
      }
    });

    EventBus.on(EventNames.resumeGame, (_: any) => {
      this.rust.resumeMusic();
    });

    EventBus.on(EventNames.onNoteInteraction, (data: NotePrecision) => {
      const interaction: OnNoteMessage = {} as OnNoteMessage;
      interaction.precision = data;
      this.rust.onInteractNote(interaction);
    });

    EventBus.on(EventNames.musicEnd, (_: any) => {
      this.rust.endGameRust();
    });
  }

  public async ngOnDestroy(): Promise<void> {
    this.phaserRef.game.destroy(true, false);
    try {
      await this.rust.stopMusic();
    } catch (error) { 
      console.error("Something went wrong, but the music is not playing..."); 
    }
    await this.rust.unlistenMidiNotes();
    this.rust.releaseOcarina();
    (Object.keys(EventNames) as Array<keyof typeof EventNames>).map((event) => EventBus.off(event));
  }

  public returnToGameMenu(): void {
    this.router.navigate(['menu-gamificada']);
  }

// --- Phaser methods
  public addNoteOnGame = (note: MidiSignal) => {
    if(note.state) this.gameScene?.createNote(note);
  }

  public pauseMusic() {
    this.gameScene.pauseGame();
  }

  public resumeMusic() {
    this.gameScene.resumeGame();
  }

  public get isMusicPaused(): boolean {
    if(!this.gameScene) return true;
    return this.gameScene.isGamePaused;
  }

}