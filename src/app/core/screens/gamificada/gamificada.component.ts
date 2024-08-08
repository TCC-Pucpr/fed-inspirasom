import { AfterViewInit, Component, OnDestroy, OnInit, ViewChild } from '@angular/core';
import { Router } from '@angular/router';
import { GameComponent } from "./game/game.component";
import { MusicService } from '../../services/music-service/music.service';
import { CommonModule } from '@angular/common';
import { MusicSheet } from '../../model/MusicSheet.model';
import { GameScene } from './game/scenes/Game.scene';
import { EventBus } from './game/events/EventBus';
import { EventNames } from './game/events/EventNames.enum';

@Component({
  selector: 'app-gamificada',
  standalone: true,
  imports: [CommonModule, GameComponent],
  templateUrl: './gamificada.component.html',
  styleUrl: './gamificada.component.scss'
})
export class GamificadaComponent implements OnInit, OnDestroy {

  @ViewChild(GameComponent) phaserRef!: GameComponent;
  private gameScene: GameScene;

  constructor(
    private router: Router,
    protected musicService: MusicService
  ) {
  }
  
  ngOnInit(): void {
    EventBus.on(EventNames.gameSceneReady, (scene: GameScene) => {
      this.gameScene = scene;
    });
  }

  ngOnDestroy(): void {
    this.phaserRef.game.destroy(true, false);
  }
  
  public redirectDashboards(): void {
    this.router.navigate(['dashboards']);
  }

  public addNoteOnGame() {
    this.gameScene.createNote("A3");
    this.gameScene.createNote("Ab3");
  }

  public pauseMusic() {
    this.gameScene.pauseGame();
  }

  public get isMusicPaused(): boolean {
    if(!this.gameScene) return true;
    return this.gameScene.isGamePaused;
  }

  public resumeMusic() {
    this.gameScene.resumeGame();
  }

}
