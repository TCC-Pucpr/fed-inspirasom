import { Component, OnDestroy, OnInit, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { InputNumberModule } from 'primeng/inputnumber';
import { FormsModule } from '@angular/forms';
import { SidebarService } from '../../services/sidebarService/sidebar.service';
import { SidebarComponent } from "../components/sidebar/sidebar.component";
import { ButtonModule } from 'primeng/button';
import { RippleModule } from 'primeng/ripple';
import { ActivatedRoute, Router } from '@angular/router';
import { RustService } from '../../services/rust/rust.service';
import { MusicService } from '../../services/musicService/music.service';
import { MidiMusic } from '../../model/MidiMusic';
import { MidiState } from '../../model/MidiState';

import * as fw from 'three';

@Component({
  selector: 'app-gamificada',
  standalone: true,
  imports: [
    CommonModule, 
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

  public row: number = 0;

  private musicState: MidiState;
  private musicData: MidiMusic;

  constructor(
    protected sidebarService: SidebarService,
    private router: Router,
    private route: ActivatedRoute,
    private rust: RustService,
    private musicService: MusicService
  ) {
  }
  
  public ngOnInit(): void {

    const queryParam = this.route.snapshot.queryParamMap.get('id');
    if(!queryParam) this.router.navigate(['menu-gamificada']);
    const musicId = parseInt(queryParam!);
    this.musicData = this.musicService.getMusicById(musicId);
  }

  public async ngOnDestroy(): Promise<void> {
    try {
      if(this.musicState != "PAUSED") await this.rust.stopMusic();
    } catch (error) { 
      console.log("Something went wrong, but the music is not playing..."); 
    }
    await this.rust.unlistenMidiNotes();
    this.rust.releaseOcarina();
  }

  public returnToGameMenu(): void {
    this.router.navigate(['menu-gamificada']);
  }

}

const scene = new fw.Scene();
const camera = new fw.PerspectiveCamera( 75, window.innerWidth/ window.innerHeight, 0.1, 1000);
const renderer = new fw.WebGLRenderer();
renderer.setSize( window.innerWidth, window.innerHeight );
document.body.appendChild(renderer.domElement);

const geometry = new fw.BoxGeometry(1, 1, 1);
const material = new fw.MeshBasicMaterial({ color: 0x00ff00 });
const cube = new fw.Mesh( geometry, material );
scene.add(cube);
camera.position.z = 5;

function animate() {
  cube.rotation.x += 0.1;
  cube.rotation.y += 0.1;
  renderer.render(scene, camera);
}

renderer.setAnimationLoop(animate);