import { CommonModule } from '@angular/common';
import { Component, Input, OnDestroy, OnInit } from '@angular/core';
import { ButtonModule } from 'primeng/button';
import { SidebarModule } from 'primeng/sidebar';
import { SidebarService } from '../../../services/sidebarService/sidebar.service';
import { ThemeService } from '../../../services/themeService/theme.service';
import { RippleModule } from 'primeng/ripple';
import { SelectButtonModule } from 'primeng/selectbutton';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { DialogService, DynamicDialogRef } from 'primeng/dynamicdialog';
import { UserProfileComponent } from '../../user-profile/user-profile.component';
import { PdfService } from '../../../services/pdfService/pdf.service';
import { OverlayComponent } from "../overlay/overlay.component";

@Component({
  selector: 'app-sidebar',
  standalone: true,
  imports: [
    CommonModule,
    SidebarModule,
    ButtonModule,
    RippleModule,
    FormsModule,
    SelectButtonModule,
    OverlayComponent
],
  providers: [
    DialogService
  ],

  templateUrl: './sidebar.component.html',
  styleUrl: './sidebar.component.scss'
})
export class SidebarComponent implements OnInit, OnDestroy {

  protected isVisible: boolean;
  protected isOverlayVisible: boolean = false;

  constructor(
    protected barService: SidebarService,
    protected themeService: ThemeService,
    private router: Router,
    public dialogService: DialogService
  ) {
    this.barService.sidebarStatus.subscribe(state => this.isVisible = state);
  }

  public ngOnInit(): void {
    const wrapper = document.getElementById("main-wrapper");
    if(wrapper) {
      // coloca um padding na esquerda quando a sidebar existir
      const bodyStyles = window.getComputedStyle(document.body);
      const width = (bodyStyles.getPropertyValue('--sidebarWidth'));
      const margin = (bodyStyles.getPropertyValue('--sidebarMarginRight'));
      wrapper.style.setProperty("padding-left", `calc(${width} + ${margin})`);
    }
  }

  public ngOnDestroy(): void {
    const wrapper = document.getElementById("main-wrapper");
    if(wrapper) wrapper.style.setProperty("padding-left", '0px');
  }

  public navigateLogin() {
    this.router.navigate(['login']);
  }

  public navigateGame() {
    this.router.navigate(['gamificada']);
  }

  public navigateDash(): void {
    this.router.navigate(['dashboards']);
  }

  public navigateMenuGamificada(): void {
    this.router.navigate(['menu-gamificada']);
  }

  public openUserProfile() {
    this.dialogService.open(UserProfileComponent, { header: 'Perfil', width: 'fit-content' });
  }

  public get isShowingScreenshot(): boolean {
    return this.router.url.includes('dashboard');
  }

  public async screenshot() {
    this.isOverlayVisible = true;
    const element = document.getElementById("docBody");
    if(element) {
      await PdfService.saveAsPdf(element);
    }
    this.isOverlayVisible = false;
  }

}
