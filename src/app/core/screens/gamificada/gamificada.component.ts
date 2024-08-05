import { Component, OnDestroy, ViewChild } from '@angular/core';
import { Router } from '@angular/router';
import { GameComponent } from "./game/game.component";
import Phaser from 'phaser';

@Component({
  selector: 'app-gamificada',
  standalone: true,
  imports: [GameComponent],
  templateUrl: './gamificada.component.html',
  styleUrl: './gamificada.component.scss'
})
export class GamificadaComponent implements OnDestroy {


  @ViewChild(GameComponent) phaserRef!: GameComponent;

  constructor(
    private router: Router
  ) {

  }
  
  public redirectDashboards(): void {
    this.router.navigate(['dashboards']);
  }

  ngOnDestroy(): void {
    this.phaserRef.game.destroy(true, false);
  }

}
