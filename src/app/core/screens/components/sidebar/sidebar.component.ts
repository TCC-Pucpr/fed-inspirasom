import { CommonModule } from '@angular/common';
import { Component, OnDestroy, OnInit } from '@angular/core';
import { ButtonModule } from 'primeng/button';
import { SidebarModule } from 'primeng/sidebar';
import { SidebarService } from '../../../services/sidebar-service/sidebar.service';
import { ThemeService } from '../../../services/theme-service/theme.service';
import { RippleModule } from 'primeng/ripple';
import { SelectButtonModule } from 'primeng/selectbutton';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';

@Component({
  selector: 'app-sidebar',
  standalone: true,
  imports: [
    CommonModule,
    SidebarModule,
    ButtonModule,
    RippleModule,
    FormsModule,
    SelectButtonModule
  ],
  templateUrl: './sidebar.component.html',
  styleUrl: './sidebar.component.scss'
})
export class SidebarComponent implements OnInit, OnDestroy {

  protected isVisible: boolean;

  constructor(
    protected barService: SidebarService,
    protected themeService: ThemeService,
    private router: Router,
  ) {
    this.barService.sidebarStatus.subscribe(state => this.isVisible = state);
  }

public ngOnInit(): void {
  const wrapper = document.getElementById("main-wrapper");
  if(wrapper) {
    const bodyStyles = window.getComputedStyle(document.body);
    wrapper.style.setProperty("padding-left", bodyStyles.getPropertyValue('--sidebarWidth'));
  }
}

public ngOnDestroy(): void {
  const wrapper = document.getElementById("main-wrapper");
  if(wrapper) wrapper.style.setProperty("padding-left", '0px');
}
  public navigateGame() {
    this.router.navigate(['gamificada']);
  }

  public navigateDash(): void {
    this.router.navigate(['dashboards']);
  }

}
