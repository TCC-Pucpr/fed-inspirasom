import { Component, Input } from '@angular/core';

@Component({
  selector: 'app-numeric-visualization',
  standalone: true,
  imports: [],
  templateUrl: './numeric-visualization.component.html',
  styleUrl: './numeric-visualization.component.scss'
})
export class NumericVisualizationComponent {

  @Input() numericInfo: number;

}
