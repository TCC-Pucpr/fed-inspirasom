import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { RustDataSourceService } from './core/services/rust/dataSource/rust-dataSource.service';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {

    constructor(
        private rustInvoker: RustDataSourceService
    ) {
    }

    greetingMessage = "";
    running = false;

    greet(event: SubmitEvent, name: string): void {
        event.preventDefault();

        if (this.running) {
            this.rustInvoker.stop_midi();
        } else {
            this.rustInvoker.connect_midi();
        }
        this.running = !this.running;
    }
}
