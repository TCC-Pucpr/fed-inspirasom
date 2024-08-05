import { Component } from '@angular/core';
import { Router } from '@angular/router';

@Component({
  selector: 'app-menu-dashboards',
  standalone: true,
  imports: [],
  templateUrl: './menu-dashboards.component.html',
  styleUrl: './menu-dashboards.component.scss'
})
export class MenuDashboardsComponent {

  constructor(
    private router: Router
  ) {

  }

  public redirectGame(): void {
    this.router.navigate(['gamificada']);
  }

}
