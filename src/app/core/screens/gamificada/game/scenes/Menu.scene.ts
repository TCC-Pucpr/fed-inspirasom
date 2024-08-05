export class MenuScene extends Phaser.Scene {
    constructor() {
        super({key: 'menu'});
    }

    preload() {
        this.add.text(450, 250, 'Menu!').setOrigin(0.5, 0.5);
    }

    create() {
        
    }
}