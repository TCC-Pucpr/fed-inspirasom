import { CommonModule } from '@angular/common';
import { Component, Input } from '@angular/core';
import { ProgressSpinnerModule } from 'primeng/progressspinner';


@Component({
  selector: 'app-overlay',
  standalone: true,
  imports: [
    ProgressSpinnerModule,
    CommonModule
  ],
  templateUrl: './overlay.component.html',
  styleUrl: './overlay.component.scss'
})
export class OverlayComponent {

  @Input() shouldAppear: boolean = false;

}
