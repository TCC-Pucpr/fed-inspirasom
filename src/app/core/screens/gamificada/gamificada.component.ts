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

  public row: number = 0;
  private processNotes = (note: any) => {
    console.log(note);
    this.addNoteOnGame(note);
  }

  constructor(
    protected sidebarService: SidebarService,
    private router: Router,
    private route: ActivatedRoute,
    private rust: RustService
  ) {
  }
  
  public async ngOnInit(): Promise<void> {
    const musicId = this.route.snapshot.queryParamMap.get('id');
    if(!musicId) this.router.navigate(['menu-gamificada']);    
    // await this.rust.startMusic(musicId!);
    // await this.rust.getMidiNotes(this.processNotes);

    EventBus.on(EventNames.gameSceneReady, (scene: GameScene) => {
      this.gameScene = scene;
    });

    EventBus.on(EventNames.exitGame, (_: any) => {
      this.router.navigate(['menu-gamificada']);
    })
  }

  public async ngOnDestroy(): Promise<void> {
    this.phaserRef.game.destroy(true, false);
    await this.rust.stopMusic();
  }

  public returnToGameMenu(): void {
    this.router.navigate(['menu-gamificada']);
  }

// --- Phaser methods
  public addNoteOnGame(note: MidiSignal) {
    this.gameScene.createNote(note.note_index, note.is_bmol);
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
