import { CommonModule } from '@angular/common';
import { Component, Input, OnInit } from '@angular/core';
import { DadosNota, PartituraNotas } from '../../../../../model/partituraNotas.model';

@Component({
  selector: 'app-secao-nota',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './secao-nota.component.html',
  styleUrl: './secao-nota.component.scss'
})

export class SecaoNotaComponent implements OnInit {

  @Input() dadosNota: DadosNota = { index: 0, isBmol: false };

  public readonly defaultNumberOfLines = 11;
  public readonly offSet = 4;
  public lines: number = this.defaultNumberOfLines;
  public index: number = 0;
  public isBmol: boolean = false;

  ngOnInit(): void {
    this.index = this.dadosNota.index;
    this.isBmol = this.dadosNota.isBmol;

    this.index += this.offSet;
    if(this.index <= PartituraNotas.blankNote) this.index = PartituraNotas.blankNote;
    if(this.index >= this.defaultNumberOfLines){
      this.lines += this.index - this.defaultNumberOfLines+1;
      this.index = this.lines-1;
    }

  }

}
