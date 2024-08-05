export class GameScene extends Phaser.Scene {

    public pressArea: Phaser.GameObjects.Rectangle; 
    public wrongPressArea: Phaser.GameObjects.Rectangle; 
    public limit: Phaser.GameObjects.Rectangle;
    public notes: Phaser.Physics.Arcade.Group;
    public readonly noteSprite: string = "note";
    
    public score: number;
    public multiplier: number;
    public chainCount: number;

    public inputs: Phaser.Types.Input.Keyboard.CursorKeys | undefined;
    public isPressed: boolean | undefined = false;
    public scoreText: Phaser.GameObjects.Text;
    public multiplierText: Phaser.GameObjects.Text;
    public chainText: Phaser.GameObjects.Text;

    constructor() {
        super({key: 'game'});
    }

    preload() {
        this.add.image(0, 0, 'background').setOrigin(0, 0);
        this.limit = this.add.rectangle(275, 127, 2, 457).setOrigin(0,0);
        this.pressArea = this.add.rectangle(287, 127, 80, 457).setOrigin(0,0);
        this.wrongPressArea = this.add.rectangle(367, 127, 650, 457).setOrigin(0,0);
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
        this.add.text(0, 0, 'Press [space] to hit the note', { color: 'white' }).setOrigin(0, 0);

        this.notes = this.physics.add.group();
        for(let i = 0; i < 5; i++){
            this.createNote(i);
        }
    }

    create() {

    }

    override update() {
        this.physics.overlap(this.notes, this.limit, this.removeNote, undefined, this);
        this.physics.overlap(this.notes, this.pressArea, this.scoredNote, undefined, this);
        this.physics.overlap(this.notes, this.wrongPressArea, this.poorNote, undefined, this);
        if(this.isPressed && this.inputs?.space.isUp) {
            this.isPressed = false;
        }
        this.multiplier = this.getCurrentMultiplier(this.chainCount);
        this.scoreText.setText(`Score: ${this.score}`);
        this.multiplierText.setText(`x${this.multiplier}`);
        this.chainText.setText(`Chain: ${this.chainCount}`);
    }

    public poorNote(area:any, note: any) { 
        if(!this.isPressed && this.inputs?.space.isDown) {
            this.isPressed = true;
            note.destroy();
            this.createNote();
            this.score -= 20;
            this.chainCount = 1;
        }
    }

    public scoredNote(area: any, note: any){
        if(!this.isPressed && this.inputs?.space.isDown) {
            this.isPressed = true;
            note.destroy();
            this.createNote();
            this.score += 10*this.multiplier;
            this.chainCount+=1;
        }
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

    public createNote(y: number = -1): void {
        if (y === -1) {
            y = Phaser.Math.Between(0, 16)*20;
        } else {
            y *= 20;
        }
        y += 203;
        this.notes.create(980, y, 'note').setVelocityX(-100);
    }

    public removeNote(limit: any, note: any): void {
        note.destroy();
        this.createNote();
        this.score -= 10;
        this.multiplier = 1;
        this.chainCount = 0;
    }

}