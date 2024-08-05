import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { GameComponent } from "./game/game.component";

@Component({
  selector: 'app-gamificada',
  standalone: true,
  imports: [GameComponent],
  templateUrl: './gamificada.component.html',
  styleUrl: './gamificada.component.scss'
})
export class GamificadaComponent {

  constructor(
    private router: Router
  ) {

  }

  public redirectDashboards(): void {
    this.router.navigate(['dashboards']);
  }

}
