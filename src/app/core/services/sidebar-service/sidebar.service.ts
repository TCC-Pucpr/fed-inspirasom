import { EventEmitter, Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class SidebarService {

  public sidebarStatus: EventEmitter<boolean> = new EventEmitter(false);
  private isSidebarVisible: boolean = false;

  constructor() { }

  public setInvisible(): void {
    this.isSidebarVisible = false;
    this.sidebarStatus.emit(false);
  }

  public setVisible(): void {
    this.isSidebarVisible = true;
    this.sidebarStatus.emit(true);
  }

  public get isVisible(): boolean {
    return this.isSidebarVisible;
  }

}
