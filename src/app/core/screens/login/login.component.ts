import { Component } from '@angular/core';
import { SidebarComponent } from "../components/sidebar/sidebar.component";

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [SidebarComponent],
  templateUrl: './login.component.html',
  styleUrl: './login.component.scss'
})
export class LoginComponent {

}
