export interface DadosNota {
    index: number, 
    isBmol: boolean
}

export class PartituraNotas {
    static readonly notas = {
        "G3": { index: 10, isBmol: false },
        "Ab3": { index: 9, isBmol: true },
        "A3": { index: 9, isBmol: false },
        "Bb3": { index: 8, isBmol: true },
        "B3": { index: 8, isBmol: false },
        "C4": { index: 7, isBmol: false }, 
        "Db4": { index: 6, isBmol: true },
        "D4": { index: 6, isBmol: false },
        "Eb4": { index: 5, isBmol: true },
        "E4": { index: 5, isBmol: false },
        "F4": { index: 4, isBmol: false },
        "Gb4": { index: 3, isBmol: true },
        "G4": { index: 3, isBmol: false },
        "Ab4": { index: 2, isBmol: true },      
        "A4": { index: 2, isBmol: false },
        "Bb4": { index: 1, isBmol: true },
        "B4": { index: 1, isBmol: false },
        "C5": { index: 0, isBmol: false },
        "None": { index: -10, isBmol: false },
    }
    static readonly blankNote = -10;
}