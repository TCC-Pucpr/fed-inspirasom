import { Component, OnDestroy, OnInit, ViewChild } from '@angular/core';
import { GameComponent } from "./game/game.component";
import { CommonModule } from '@angular/common';
import { GameScene } from './game/scenes/Game.scene';
import { EventBus } from './game/events/EventBus';
import { EventNames } from './game/events/EventNames.enum';
import { InputNumberModule } from 'primeng/inputnumber';
import { FormsModule } from '@angular/forms';
import { SidebarService } from '../../services/sidebar-service/sidebar.service';
import { SidebarComponent } from "../components/sidebar/sidebar.component";
import { ButtonModule } from 'primeng/button';
import { RippleModule } from 'primeng/ripple';

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

  constructor(
    protected sidebarService: SidebarService
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

// --- Phaser methods
  public addNoteOnGame(row: number = 0, isBmol: boolean) {
    this.gameScene.createNote(row, isBmol);
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
