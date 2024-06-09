import { Component } from '@angular/core';
import { Router } from '@angular/router';

@Component({
  selector: 'app-menu-gamificado',
  standalone: true,
  imports: [],
  templateUrl: './menu-gamificado.component.html',
  styleUrl: './menu-gamificado.component.scss'
})
export class MenuGamificadoComponent {

  constructor(
    private router: Router
  ){}

  public redirecionaPartitura() {
    this.router.navigate(['partitura']);
  }

}
