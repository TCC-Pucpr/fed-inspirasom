import { Component, OnInit } from '@angular/core';
import { SidebarComponent } from "../components/sidebar/sidebar.component";
import { ButtonModule } from 'primeng/button';
import { Router } from '@angular/router';
import { DialogService } from 'primeng/dynamicdialog';
import { PreferenciasGamificadaComponent } from './components/preferencias-gamificada/preferencias-gamificada.component';
import { RustService } from '../../services/rust/rust.service';
import { MidiMusic } from '../../model/MidiMusic';
import { CommonModule } from '@angular/common';
import { MusicService } from '../../services/musicService/music.service';
import { MidiSignal } from '../../model/MidiSignal';

@Component({
  selector: 'app-menu-gamificada',
  standalone: true,
  imports: [
    SidebarComponent,
    ButtonModule,
    CommonModule
  ],
  providers: [
    DialogService
  ],
  templateUrl: './menu-gamificada.component.html',
  styleUrl: './menu-gamificada.component.scss'
})
export class MenuGamificadaComponent implements OnInit {

  protected musicList: MidiMusic[];

  constructor(
    private router: Router,
    public dialogService: DialogService,
    private musicService: MusicService,

    private rust: RustService
  ) {

  }

  public async ngOnInit(): Promise<void> {
    this.musicList = this.musicService.getMusicList();
    if(this.musicList.length == 0) {
      this.musicList = await this.musicService.fetchMusicList();
    }
  }

  private ngOnDestroy(): void {
    this.rust.releaseOcarina();
  }

  public openPreferenciasGamificada(): void {
    this.dialogService.open(PreferenciasGamificadaComponent, { header: 'Preferencias'} );
  }

  public selectMusic(music: MidiMusic): void {
    this.router.navigate(['gamificada'], { queryParams: { id: music.id }});
  }

}
