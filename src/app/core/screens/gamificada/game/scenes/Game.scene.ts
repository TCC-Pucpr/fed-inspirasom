import { MidiSignal } from "../../../../model/MidiSignal";
import { EventBus } from "../events/EventBus";
import { EventNames } from "../events/EventNames.enum";

export class GameScene extends Phaser.Scene {

    public pressArea: Phaser.GameObjects.Rectangle; 
    public wrongPressArea: Phaser.GameObjects.Rectangle; 
    public limit: Phaser.GameObjects.Rectangle;
    public notes: Phaser.Types.Physics.Arcade.SpriteWithDynamicBody[] = [];
    public readonly noteSprite: string = "note";
    
    public score: number;
    public multiplier: number;
    public chainCount: number;

    public inputs: Phaser.Types.Input.Keyboard.CursorKeys | undefined;
    public isPressed: boolean | undefined = false;
    public scoreText: Phaser.GameObjects.Text;
    public multiplierText: Phaser.GameObjects.Text;
    public chainText: Phaser.GameObjects.Text;
    public noteText: Phaser.GameObjects.Text;

    public isPaused: boolean = false;

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
    }
    
    public create() {
        EventBus.emit(EventNames.gameSceneReady, this);
        EventBus.on(EventNames.resumeGame, this.resumeGame);
        EventBus.on(EventNames.pauseGame, this.pauseGame);
        EventBus.on(EventNames.ocarinaNote, (note: MidiSignal) => { this.lastOcarinaNote = note });
        const escKey = this.input.keyboard?.addKey(Phaser.Input.Keyboard.KeyCodes.ESC)!;
        escKey.on('down', () => { EventBus.emit(EventNames.pauseGame); });
        this.scene.launch("pause");
    }

    public override update() {
        if(this.notes.length > 0) {
            const note = this.notes[0];
            this.physics.overlap(note, this.limit, this.removeNote, undefined, this);
            this.physics.overlap(note, this.pressArea, this.scoredNote, undefined, this);
            this.physics.overlap(note, this.wrongPressArea, this.poorNote, undefined, this);
        }
        if(this.isPressed && this.inputs?.space.isUp) {
            this.isPressed = false;
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
    }

    public poorNote(note: any, area: any) { 
        if(!this.isPressed && this.inputs?.space.isDown) {
            this.removeNoteFromScene(note);
            this.isPressed = true;
            this.score -= 20;
            this.chainCount = 1;
        }
    }

    public scoredNote(note: any, area: any){
        if(!this.isPressed && this.inputs?.space.isDown) {
            const noteCenter = note.x - note.width/2;
            const accScore = this.getAccScore(noteCenter, this.pressArea.x, this.pressArea.width);
            this.isPressed = true;
            this.removeNoteFromScene(note);
            this.score += (10 + accScore)*this.multiplier;
            this.chainCount+=1;
        }
    }

    public pauseGame = () => {
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

    public oldCreateNote(row: number, isBmol: boolean): void {
        if( row === -1 ) return;
        const y = 226 + (row * 13);
        const x = 980;
        const s = 1;
        const type = isBmol ? 'bmolNote' : 'note';
        try{
            const note = this.physics.add.sprite(x, y, type);
            note.setVelocityX(-100*s).setOrigin(1, 1).setSize(35, 28).setDisplaySize(35, 32);
            this.notes.push(note);
        } catch(error) { }
    }

    public createNote(row: number, isBmol: boolean): void {
        if( row === -1 ) return;
        const y = 226 + (row * 13);
        const x = 980;
        const s = 1;
        try{
            if(isBmol) {
                const note = this.physics.add.sprite(x, y+3, 'bmolNote');
                note.setVelocityX(-100*s).setOrigin(1, 1).setSize(35, 28).setDisplaySize(42, 36).setOffset(11, 4);
                this.notes.push(note);
            } else {
                const note = this.physics.add.sprite(x, y, 'note');
                note.setVelocityX(-100*s).setOrigin(1, 1).setSize(35, 28).setDisplaySize(35, 32);
                this.notes.push(note);
            }
        } catch(error) { }
    }

    public removeNoteFromScene(note: Phaser.Types.Physics.Arcade.SpriteWithDynamicBody) {
        const index = this.notes.indexOf(note);
        if( index !== -1) {
            this.notes.shift()?.destroy();
        }
    }

}