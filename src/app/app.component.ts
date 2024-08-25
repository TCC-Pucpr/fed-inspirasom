import { Component, Inject, OnInit } from '@angular/core';
import { CommonModule, DOCUMENT } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { RustDataSourceService } from './core/services/rust/dataSource/rust-dataSource.service';

import { ButtonModule } from 'primeng/button';
import { ThemeService } from './core/services/themeService/theme.service';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet, ButtonModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss'
})
export class AppComponent implements OnInit {

    constructor(
        private rustInvoker: RustDataSourceService,
        protected themeService: ThemeService
    ) { }
    
    ngOnInit(): void {
        this.themeService.initTheme();
    }

    greetingMessage = "";
    isListeningMidi = false;

    greet(event: SubmitEvent, name: string): void {
        event.preventDefault();
        if (this.isListeningMidi) {
            this.rustInvoker.stop_midi();
        } else {
            this.rustInvoker.connect_midi();
        }
        this.isListeningMidi = !this.isListeningMidi;
    }

}
