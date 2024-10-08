import { EndgameDataModel } from "../../../../model/EndgameData.model";
import { MidiSignal } from "../../../../model/MidiSignal";
import { MidiState } from "../../../../model/MidiState";
import { NotePrecision } from "../../../../model/NotePrecision.model";
import { OnNotePrecision } from "../../../../model/NotePressPrecision";
import { NoteStatusModel } from "../../../../model/NoteStatus.model";
import { EventBus } from "../events/EventBus";
import { EventNames } from "../events/EventNames.enum";

export class GameScene extends Phaser.Scene {

    public pressArea: Phaser.GameObjects.Rectangle; 
    public wrongPressArea: Phaser.GameObjects.Rectangle; 
    public limit: Phaser.GameObjects.Rectangle;
    public notes: { note: MidiSignal, body: Phaser.Types.Physics.Arcade.SpriteWithDynamicBody }[] = [];
    
    public score: number;
    public multiplier: number;
    public chainCount: number;

    public inputs: Phaser.Types.Input.Keyboard.CursorKeys | undefined;
    public scoreText: Phaser.GameObjects.Text;
    public multiplierText: Phaser.GameObjects.Text;
    public chainText: Phaser.GameObjects.Text;
    public noteText: Phaser.GameObjects.Text;

    public musicState: MidiState;
    public isPaused: boolean = false;
    public isEndGame: boolean = false;

    public noteStatus: NoteStatusModel = {} as NoteStatusModel;
    public lastOcarinaNote: MidiSignal = {} as MidiSignal;
    
    constructor(
    ) {
        super({key: 'game'});
    }

    public preload() {
        this.add.image(0, 0, 'background').setOrigin(0, 0);
        this.limit = this.add.rectangle(0, 127, 267, 457).setOrigin(0,0);
        this.pressArea = this.add.rectangle(297, 127, 30, 457).setOrigin(0,0);
        this.wrongPressArea = this.add.rectangle(327, 127, 80, 457).setOrigin(0,0);
        this.physics.add.existing(this.limit, true);
        this.physics.add.existing(this.pressArea, true);
        this.physics.add.existing(this.wrongPressArea, true);
        this.score = 0;
        this.chainCount = 0;
        this.multiplier = 1;

        this.inputs = this.input.keyboard?.createCursorKeys();
        this.scoreText = this.add.text(50, 50, '', { color: 'white' }).setOrigin(0, 0);
        this.multiplierText = this.add.text(50, 75, '', { color: 'white' }).setOrigin(0, 0);
        this.chainText = this.add.text(50, 100, '', { color: 'white' }).setOrigin(0, 0);
        this.noteText = this.add.text(50, 125, '', { color: 'white' }).setOrigin(0, 0);
        this.add.text(0, 0, `Press [space] to hit the note, press [ESC] to pause`, { color: 'white' }).setOrigin(0, 0);

        this.noteStatus = { hitNotes: 0, missedNotes: 0, poorNotes: 0 };
    }
    
    public create() {
        EventBus.emit(EventNames.gameSceneReady, this);
        EventBus.on(EventNames.resumeGame, this.resumeGame);
        EventBus.on(EventNames.pauseGame, this.pauseGame);
        EventBus.on(EventNames.ocarinaNote, this.checkForNote);
        EventBus.on(EventNames.musicStateChange, this.treatStates);
        const escKey = this.input.keyboard?.addKey(Phaser.Input.Keyboard.KeyCodes.ESC)!;
        escKey.on('down', () => { EventBus.emit(EventNames.pauseGame); });
        this.scene.launch("endScreen");
        this.scene.launch("pause");
    }

    public checkForNote = (playedNote: MidiSignal) => {
        this.lastOcarinaNote = playedNote;
        try {
            if(this.notes.length > 0) {
                const note = this.notes[0];
                console.log(`body: [${note.note.note_name}], current: [${playedNote.note_name}]`);
                if(note.note.note_name === playedNote.note_name) {
                    this.physics.overlap(note.body, this.limit, this.removeNote, undefined, this);
                    this.physics.overlap(note.body, this.pressArea, this.scoredNote, undefined, this);
                    this.physics.overlap(note.body, this.wrongPressArea, this.poorNote, undefined, this);
                }
            }
        } catch (err) { };
    }

    public override update() {
        if(this.notes.length > 0) {
            const note = this.notes[0];
            this.physics.overlap(note.body, this.limit, this.removeNote, undefined, this);
        } else if (this.notes.length === 0 && this.isEndGame) {
            this.openEndScreen();
        }
        this.multiplier = this.getCurrentMultiplier(this.chainCount);
        this.scoreText.setText(`Score: ${this.score}`);
        this.multiplierText.setText(`x${this.multiplier}`);
        this.chainText.setText(`Chain: ${this.chainCount}`);
        this.noteText.setText(`Note: ${this.lastOcarinaNote.note_name}`);
    }

    public removeNote(note: any, limit: any): void {
        this.removeNoteFromScene(note);
        this.score -= 10;
        this.multiplier = 1;
        this.chainCount = 0;
        this.noteStatus.missedNotes++;
        EventBus.emit(EventNames.onNoteInteraction, NotePrecision.Miss)
    }

    public poorNote(note: any, area: any) { 
        this.removeNoteFromScene(note);
        this.score -= 20;
        this.chainCount = 1;
        this.noteStatus.poorNotes++;
        EventBus.emit(EventNames.onNoteInteraction, NotePrecision.EarlyMiss)
    }

    public scoredNote(note: any, area: any){
        const noteCenter = note.x - note.width/2;
        const accScore = this.getAccScore(noteCenter, this.pressArea.x, this.pressArea.width);
        this.removeNoteFromScene(note);
        this.score += (10 + accScore)*this.multiplier;
        this.chainCount++;
        this.noteStatus.hitNotes++;
        EventBus.emit(EventNames.onNoteInteraction, NotePrecision.Middle)
    }

    public treatStates = (state: MidiState) => {
        this.musicState = state;
        try{
            if(state === "FINISHED") {
                this.isEndGame = true;
                
            }
        } catch (error){ }
    }

    public openEndScreen() {
        const callback = () => {
            this.scene.bringToTop("endScreen");
        };
        callback.bind(this);
        setTimeout(callback, 1500);
        const data = {} as EndgameDataModel;
        data.noteStatus = this.noteStatus;
        data.score = this.score;
        EventBus.emit(EventNames.musicEnd, data);
        this.scene.pause();
    }

    public pauseGame = () => {
        if(this.isEndGame && this.notes.length === 0) {
            return;
        }
        try{
            this.isPaused = true;
            this.scene.bringToTop("pause");
            this.scene.pause();
        } catch (error){ }
    }

    public resumeGame = () => {
        try{
            this.isPaused = false;
            this.scene.bringToTop("game");
            this.scene.resume("game");
        } catch (error){ }
    }

    public get isGamePaused(): boolean {
        return this.game.isPaused;
    }

    public getCurrentMultiplier(chain: number): number {
        if(chain < 10) {
            return 1;
        }
        if(chain <= 15){
            return 2;
        }
        if(chain <= 20){ 
            return 4;
        }
        return 8;
    }

    public getAccScore(notePos: number, areaX: number, areaWidth: number): number {
        const sectorSize = areaWidth/3;
        if(areaX+(sectorSize*3) > notePos && notePos > areaX+(sectorSize*2)) {
            return 5;
        }
        if(areaX+(sectorSize*2) > notePos && notePos > areaX+(sectorSize)) {
            return 10;
        } else {
            return 5;
        }
    }

    public createNote(note: MidiSignal): void {
        if( note.note_index === -1 ) return;
        const y = 226 + (note.note_index * 13);
        const x = 980;
        const s = 1;
        try{
            if(note.is_bmol) {
                const body = this.physics.add.sprite(x, y+3, 'bmolNote');
                body.setVelocityX(-100*s).setOrigin(1, 1).setSize(35, 28).setDisplaySize(42, 36).setOffset(11, 4);
                this.notes.push({ note, body });
            } else {
                const body = this.physics.add.sprite(x, y, 'note');
                body.setVelocityX(-100*s).setOrigin(1, 1).setSize(35, 28).setDisplaySize(35, 32);
                this.notes.push({ note, body });
            }
        } catch(error) { }
    }

    public removeNoteFromScene(note: Phaser.Types.Physics.Arcade.SpriteWithDynamicBody) {
        if(this.notes.filter((data) => data.body === note)) {
            this.notes.shift()?.body.destroy();
        }
    }

}