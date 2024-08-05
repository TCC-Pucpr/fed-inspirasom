export class BootUtils extends Phaser.Scene {
    constructor() {
        super({ key: 'boot' });
    }

    preload() {
        this.load.setPath('assets/gameAssets');
        this.load.image('background', 'background.png');
        this.load.svg('note', 'note.svg');
    }

    create() {
        this.game.scene.start('game');
    }

}