import { Component } from '@angular/core';
import { SecaoNotaComponent } from './secao-nota/secao-nota.component';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-partitura',
  standalone: true,
  imports: [
    CommonModule,
    SecaoNotaComponent
  ],
  templateUrl: './partitura.component.html',
  styleUrl: './partitura.component.scss'
})
export class PartituraComponent {

}
