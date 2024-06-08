import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { invoke } from "@tauri-apps/api/tauri";
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
    ) { }

  greetingMessage = "";
  running = false;

  greet(event: SubmitEvent, name: string): void {
    event.preventDefault();

    if(this.running) {
      invoke<boolean>("stop_connection").then((result) => {
        console.log("Received stop result")
      });
    } else {
      invoke<void>("connect_arduino_midi").then((text) => {
        console.log("Received")
      });
    }
    this.running = !this.running;
  }
}
