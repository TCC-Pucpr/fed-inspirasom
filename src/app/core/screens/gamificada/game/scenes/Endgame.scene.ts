import { EndgameDataModel } from "../../../../model/EndgameData.model";
import { NoteStatusModel } from "../../../../model/NoteStatus.model";
import { EventBus } from "../events/EventBus";
import { EventNames } from "../events/EventNames.enum";

export class EndgameScene extends Phaser.Scene {

    public musicName: string;
    private titleText: Phaser.GameObjects.Text;
    private notesText: Phaser.GameObjects.Text;
    public notes: NoteStatusModel = {} as NoteStatusModel;
    public totalNotes = 0;
    public totalScore = 0;

    constructor() {
        super({key: 'endScreen'});
    }

    public preload() {

    }

    public create() {
        this.add.rectangle(0, 0, 1280, 720, 0x000000, 0.7).setOrigin(0, 0);
        this.titleText = this.add.text(1280/2, 720/4, 'The end').setOrigin(0.5, 0.5);
        this.notesText = this.add.text(1280/2, 720/3, '').setOrigin(0.5, 0.5);
        const quitButton = this.add.rectangle(1280/2, 720/2, 100, 50, 0xffffff).setOrigin(0.5, 0.5);
        const quitText = this.add.text(0, 0, 'Quit', {color: '0x000000'}).setOrigin(0.5, 0.5);
        Phaser.Display.Align.In.Center(quitText, quitButton);
        
        quitButton.setInteractive();
        
        quitButton.on('pointerdown', () => {
            EventBus.emit(EventNames.exitGame);
        });
        EventBus.on(EventNames.musicEnd, (data: EndgameDataModel) => {
            this.notes = data.noteStatus;
            this.totalNotes = this.notes.missedNotes + this.notes.poorNotes + this.notes.hitNotes;
            this.totalScore = data.score;
        });
        EventBus.emit(EventNames.endSceneReady, this);
    }

    public override update() {
        this.titleText.setText(`${this.musicName}`);
        this.notesText.setText(`${this.notes.hitNotes}/${this.totalNotes} acertos! | Pontuação: ${this.totalScore}`);
    }

}