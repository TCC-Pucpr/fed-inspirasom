import { Component } from '@angular/core';
import { SidebarComponent } from "../components/sidebar/sidebar.component";
import { ButtonModule } from 'primeng/button';
import { Router } from '@angular/router';
import { DialogService } from 'primeng/dynamicdialog';
import { PreferenciasGamificadaComponent } from './components/preferencias-gamificada/preferencias-gamificada.component';

@Component({
  selector: 'app-menu-gamificada',
  standalone: true,
  imports: [
    SidebarComponent,
    ButtonModule,
  ],
  providers: [
    DialogService
  ],
  templateUrl: './menu-gamificada.component.html',
  styleUrl: './menu-gamificada.component.scss'
})
export class MenuGamificadaComponent {

  constructor(
    private router: Router,
    public dialogService: DialogService
  ) {

  }

  public openPreferenciasGamificada(): void {
    this.dialogService.open(PreferenciasGamificadaComponent, { header: 'Preferencias'} );
  }

}
