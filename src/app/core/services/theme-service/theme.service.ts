import { DOCUMENT } from '@angular/common';
import { EventEmitter, Inject, Injectable } from '@angular/core';
import { DataService, StorageKeys } from '../dataService/data.service';

@Injectable({
  providedIn: 'root'
})
export class ThemeService {

  public onThemeChange: EventEmitter<String> = new EventEmitter();
  private theme: string = "light";
  public readonly THEME_KEY = "THEME";

  constructor(
    @Inject(DOCUMENT) private document: Document,
    private storage: DataService
  ) { }

  public initTheme(): void {
    const theme = this.storage.get(StorageKeys.theme) as "light"|"dark"|null;
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
    this.storage.set(StorageKeys.theme, theme);
    this.theme = theme;

    this.onThemeChange.emit(theme);

    const themeLink = this.document.getElementById("app-theme") as HTMLLinkElement;
    if(themeLink) {
        themeLink.href = `${theme}.css`;
    }
  }

}
