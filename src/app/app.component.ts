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
  ) { }

  public greetingMessage: string = "";

  public async updateMessage(event: SubmitEvent, message: string): Promise<void>{
    this.greetingMessage = await this.rustInvoker.greet(event, message);
  }
  
}
