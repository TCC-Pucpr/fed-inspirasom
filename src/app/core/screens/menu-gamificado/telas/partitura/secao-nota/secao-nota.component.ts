import { CommonModule } from '@angular/common';
import { Component, Input, OnInit } from '@angular/core';

@Component({
  selector: 'app-secao-nota',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './secao-nota.component.html',
  styleUrl: './secao-nota.component.scss'
})

export class SecaoNotaComponent implements OnInit {

  @Input() indexNota: number = 0;

  public readonly defaultNumberOfLines = 9;
  public lines: number = this.defaultNumberOfLines;

  ngOnInit(): void {
    if(this.indexNota >= this.defaultNumberOfLines){
      this.lines += this.indexNota - this.defaultNumberOfLines+1;
      this.indexNota = this.lines-1;
    }

  }

}
