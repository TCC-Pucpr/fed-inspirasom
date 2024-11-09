import { Component, OnInit } from '@angular/core';
import { SidebarComponent } from "../components/sidebar/sidebar.component";
import { ButtonModule } from 'primeng/button';
import { Router } from '@angular/router';
import { DialogService } from 'primeng/dynamicdialog';
import { RustService } from '../../services/rust/rust.service';
import { MidiMusic } from '../../model/MidiMusic';
import { CommonModule } from '@angular/common';
import { MusicService } from '../../services/musicService/music.service';
import { open } from '@tauri-apps/api/dialog';
import { DialogModule } from 'primeng/dialog';
import { InputTextModule } from 'primeng/inputtext';
import { FormControl, ReactiveFormsModule } from '@angular/forms';

@Component({
  selector: 'app-menu-gamificada',
  standalone: true,
  imports: [
    SidebarComponent,
    ButtonModule,
    CommonModule,
    DialogModule,
    InputTextModule,
    ReactiveFormsModule
  ],
  providers: [
    DialogService
  ],
  templateUrl: './menu-gamificada.component.html',
  styleUrl: './menu-gamificada.component.scss'
})
export class MenuGamificadaComponent implements OnInit {

  protected musicList: MidiMusic[];
  protected newMusicPath: string = '';
  protected isFileNameModalOpen: boolean = false;
  protected newMusicName = new FormControl('');

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

  public async openPreferenciasGamificada() {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'Selecione uma musica',
        extensions: ['mid']
      }]
    });
    if (selected) {
      this.newMusicPath = selected as string;
      this.isFileNameModalOpen = true;
    }
  }

  public fecharModal() {
    this.newMusicName.reset();
    this.newMusicPath = '';
    this.isFileNameModalOpen = false;
  }

  public async confirmarModal() {
    this.newMusicName.markAllAsTouched();
    if(!this.newMusicName.valid) {
      await this.rust.addNewMusic(this.newMusicName.value as string, this.newMusicPath);
      this.musicList = [];
      await this.ngOnInit();
      this.fecharModal();
    }
  }

  public selectMusic(music: MidiMusic): void {
    this.router.navigate(['gamificada'], { queryParams: { id: music.id }});
  }

}
