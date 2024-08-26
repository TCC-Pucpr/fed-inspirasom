import { Component, OnInit } from '@angular/core';
import { SidebarComponent } from "../components/sidebar/sidebar.component";
import { ButtonModule } from 'primeng/button';
import { Router } from '@angular/router';
import { DialogService } from 'primeng/dynamicdialog';
import { PreferenciasGamificadaComponent } from './components/preferencias-gamificada/preferencias-gamificada.component';
import { RustService } from '../../services/rust/rust.service';

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
export class MenuGamificadaComponent implements OnInit {

  constructor(
    private router: Router,
    public dialogService: DialogService,
    private rust: RustService
  ) {

  }

  public ngOnInit(): void {
    this.listMusics();
  }

  public async listMusics() {
    const list = await this.rust.getMusicList();
    console.log(list);
  }

  public openPreferenciasGamificada(): void {
    this.dialogService.open(PreferenciasGamificadaComponent, { header: 'Preferencias'} );
  }

}
