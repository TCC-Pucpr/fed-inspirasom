import { ChangeDetectorRef, Component, OnDestroy, OnInit } from '@angular/core';
import { SecaoNotaComponent } from './secao-nota/secao-nota.component';
import { CommonModule } from '@angular/common';
import { RustDataSourceService } from '../../../../services/rust/dataSource/rust-dataSource.service';
import { MidiSignal } from '../../../../model/MidiSignal';
import { PartituraNotas } from '../../../../model/partituraNotas.model';
import { Router } from '@angular/router';

@Component({
  selector: 'app-partitura',
  standalone: true,
  imports: [
    CommonModule,
    SecaoNotaComponent
  ],
  templateUrl: './partitura.component.html',
  styleUrl: './partitura.component.scss'
})
export class PartituraComponent implements OnInit, OnDestroy {

  public readonly partituraSize = 30;
  public notasIndex: number[] = Array.from({ length: this.partituraSize }, (x, i) => i);
  public isScrollerRunning: boolean = false;
  public scrollerController: any;

  constructor(
    private rustInvoker: RustDataSourceService,
    private router: Router,
    private cdRef: ChangeDetectorRef
  ){  }

  ngOnInit(): void {
    this.rustInvoker.connect_midi();
    this.andUpdateNotes.bind(this);
    this.rustInvoker.listen_for_midi_note(this.andUpdateNotes);
  }

  public ngOnDestroy(): void {
    this.rustInvoker.stop_midi();
  }

  public voltaMenu() {
    this.router.navigate(['menu-gamificado']);
  }

  public toggleScroller(){
    if(this.isScrollerRunning){
      this.stopScroller();
    } else {
      this.startScroller();
    }
  }

  private startScroller(){
    this.scrollerController = setInterval(() => {
      this.notasIndex.concat(this.notasIndex.splice(0,1));
      this.notasIndex.push(-10);
    }, 100);
    this.isScrollerRunning = true;
  }

  private stopScroller(){
    clearInterval(this.scrollerController);
    this.isScrollerRunning = false;
  }  

  public andUpdateNotes(signal: MidiSignal){
    if(signal.state == 128) return;
    if(this.partituraSize >= 30) this.notasIndex.concat(this.notasIndex.splice(0,1));
    this.notasIndex.push(PartituraNotas.notas[signal.note.note]);
    this.cdRef.detectChanges();
  }

}
