import {Component, OnDestroy, OnInit, ViewChild} from '@angular/core';
import {GameComponent} from "./game/game.component";
import {CommonModule} from '@angular/common';
import {GameScene} from './game/scenes/Game.scene';
import {EventBus} from './game/events/EventBus';
import {EventNames} from './game/events/EventNames.enum';
import {InputNumberModule} from 'primeng/inputnumber';
import {FormsModule} from '@angular/forms';
import {SidebarService} from '../../services/sidebarService/sidebar.service';
import {SidebarComponent} from "../components/sidebar/sidebar.component";
import {ButtonModule} from 'primeng/button';
import {RippleModule} from 'primeng/ripple';
import {ActivatedRoute, Router} from '@angular/router';
import {RustService} from '../../services/rust/rust.service';
import {MidiSignal} from '../../model/MidiSignal';
import {PauseScene} from './game/scenes/Pause.scene';
import {MusicService} from '../../services/musicService/music.service';
import {MidiMusic} from '../../model/MidiMusic';

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
    private pauseScene: PauseScene;

    public row: number = 0;

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
        const musicId = this.route.snapshot.queryParamMap.get('id');
        if (!musicId) this.router.navigate(['menu-gamificada']);
        this.rust.startMusic(+musicId!);
        this.rust.listenMidiNotes(this.addNoteOnGame);
        this.musicData = this.musicService.getMusicById(+musicId!);

        this.rust.connect_midi();
        this.rust.listen_for_midi_note((note: MidiSignal) => {
            EventBus.emit(EventNames.ocarinaNote, note);
        });

        EventBus.on(EventNames.gameSceneReady, (scene: GameScene) => {
            this.gameScene = scene;
        });

        EventBus.on(EventNames.pauseSceneReady, (scene: PauseScene) => {
            this.pauseScene = scene;
            this.setMusicName(this.musicData.name);
        });

        EventBus.on(EventNames.exitGame, (_: any) => {
            this.returnToGameMenu();
        });

        EventBus.on(EventNames.pauseGame, (_: any) => {
            this.rust.pauseMusic();
        });

        EventBus.on(EventNames.resumeGame, (_: any) => {
            this.rust.resumeMusic();
        });

    }

    public async ngOnDestroy(): Promise<void> {
        this.phaserRef.game.destroy(true, false);
        await this.rust.stopMusic();
        await this.rust.unlistenMidiNotes();
        this.rust.stop_midi();
        EventBus.off(EventNames.gameSceneReady);
        EventBus.off(EventNames.exitGame);
        EventBus.off(EventNames.pauseGame);
        EventBus.off(EventNames.resumeGame);
    }

    public returnToGameMenu(): void {
        this.router.navigate(['menu-gamificada']);
    }

// --- Phaser methods
    public addNoteOnGame = (note: MidiSignal) => {
        if (note.state) this.gameScene?.createNote(note.note_index, note.is_bmol);
    }

    public pauseMusic() {
        this.gameScene.pauseGame();
    }

    public resumeMusic() {
        this.gameScene.resumeGame();
    }

    public get isMusicPaused(): boolean {
        if (!this.gameScene) return true;
        return this.gameScene.isGamePaused;
    }

    public setMusicName(musicName: string) {
        this.pauseScene.musicName = musicName;
    }


}
