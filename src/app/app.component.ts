import { Component, Inject } from '@angular/core';
import { CommonModule, DOCUMENT } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { RustDataSourceService } from './core/services/rust/dataSource/rust-dataSource.service';

import { ButtonModule } from 'primeng/button';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet, ButtonModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss'
})
export class AppComponent {

    constructor(
        private rustInvoker: RustDataSourceService,
        @Inject(DOCUMENT) private document: Document
    ) {
    }

    public themes = [
        "primeng/resources/themes/bootstrap4-light-blue/theme.css",
        "primeng/resources/themes/bootstrap4-light-purple/theme.css",
        "primeng/resources/themes/bootstrap4-dark-blue/theme.css",
        "primeng/resources/themes/bootstrap4-dark-purple/theme.css",
        "primeng/resources/themes/md-light-indigo/theme.css",
        "primeng/resources/themes/md-light-deeppurple/theme.css",
        "primeng/resources/themes/md-dark-indigo/theme.css",
        "primeng/resources/themes/md-dark-deeppurple/theme.css",
        "primeng/resources/themes/mdc-light-indigo/theme.css",
        "primeng/resources/themes/mdc-light-deeppurple/theme.css",
        "primeng/resources/themes/mdc-dark-indigo/theme.css",
        "primeng/resources/themes/mdc-dark-deeppurple/theme.css",
        "primeng/resources/themes/fluent-light/theme.css",
        "primeng/resources/themes/lara-light-blue/theme.css",
        "primeng/resources/themes/lara-light-indigo/theme.css",
        "primeng/resources/themes/lara-light-purple/theme.css",
        "primeng/resources/themes/lara-light-teal/theme.css",
        "primeng/resources/themes/lara-dark-blue/theme.css",
        "primeng/resources/themes/lara-dark-indigo/theme.css",
        "primeng/resources/themes/lara-dark-purple/theme.css",
        "primeng/resources/themes/lara-dark-teal/theme.css",
        "primeng/resources/themes/soho-light/theme.css",
        "primeng/resources/themes/soho-dark/theme.css",
        "primeng/resources/themes/viva-light/theme.css",
        "primeng/resources/themes/viva-dark/theme.css",
        "primeng/resources/themes/mira/theme.css",
        "primeng/resources/themes/nano/theme.css",
        "primeng/resources/themes/saga-blue/theme.css",
        "primeng/resources/themes/saga-green/theme.css",
        "primeng/resources/themes/saga-orange/theme.css",
        "primeng/resources/themes/saga-purple/theme.css",
        "primeng/resources/themes/vela-blue/theme.css",
        "primeng/resources/themes/vela-green/theme.css",
        "primeng/resources/themes/vela-orange/theme.css",
        "primeng/resources/themes/vela-purple/theme.css",
        "primeng/resources/themes/arya-blue/theme.css",
        "primeng/resources/themes/arya-green/theme.css",
        "primeng/resources/themes/arya-orange/theme.css",
        "primeng/resources/themes/arya-purple/theme.css",
        "primeng/resources/themes/nova/theme.css",
        "primeng/resources/themes/nova-alt/theme.css",
        "primeng/resources/themes/nova-accent/theme.css",
        "primeng/resources/themes/luna-amber/theme.css",
        "primeng/resources/themes/luna-blue/theme.css",
        "primeng/resources/themes/luna-green/theme.css",
        "primeng/resources/themes/luna-pink/theme.css",
        "primeng/resources/themes/rhea/theme.css"
    ]

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

    public changeTheme(newTheme: string) {
        const themeLink = this.document.getElementById("app-theme") as HTMLLinkElement;
        if(themeLink) {
            themeLink.href = `${newTheme}.css`;
        }
    }

    public changeThemeEasy(theme: number) {
        const themeLink = this.document.getElementById("app-theme") as HTMLLinkElement;
        if(themeLink) {
            themeLink.href = `${theme}.css`;
        }
    }

}
