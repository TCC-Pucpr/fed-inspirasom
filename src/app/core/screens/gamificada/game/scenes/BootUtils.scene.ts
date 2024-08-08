export class BootUtils extends Phaser.Scene {
    constructor() {
        super({ key: 'boot' });
    }

    preload() {
        this.load.setPath('assets/gameAssets');
        this.load.image('background', 'background.png');
        this.load.image('note', 'note.png');
        this.load.image('bmolNote', 'bmol.png');
    }

    create() {
        this.game.scene.start('game');
    }

}