import { DOCUMENT } from '@angular/common';
import { EventEmitter, Inject, Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class ThemeService {

  public onThemeChange: EventEmitter<String> = new EventEmitter();
  private theme: string = "light";
  public readonly THEME_KEY = "THEME";

  constructor(
    @Inject(DOCUMENT) private document: Document
  ) { }

  public initTheme(): void {
    const theme = localStorage.getItem(this.THEME_KEY) as "light"|"dark"|null;
    if(theme){
      this.setTheme(theme);
    } else {
      this.setTheme("light");
    }
  }

  public get currentTheme(): "light"|"dark" {
    return this.theme as "light"|"dark";
  }

  public setTheme(theme: "light"|"dark"): void {
    localStorage.setItem(this.THEME_KEY, theme);
    this.theme = theme;

    this.onThemeChange.emit(theme);

    const themeLink = this.document.getElementById("app-theme") as HTMLLinkElement;
    if(themeLink) {
        themeLink.href = `${theme}.css`;
    }
  }

}
