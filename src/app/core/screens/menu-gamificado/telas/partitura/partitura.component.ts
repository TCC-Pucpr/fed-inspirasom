import { ChangeDetectorRef, Component, OnDestroy, OnInit } from '@angular/core';
import { SecaoNotaComponent } from './secao-nota/secao-nota.component';
import { CommonModule } from '@angular/common';
import { RustDataSourceService } from '../../../../services/rust/dataSource/rust-dataSource.service';
import { MidiSignal } from '../../../../model/MidiSignal';
import { DadosNota, PartituraNotas } from '../../../../model/partituraNotas.model';
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

  public readonly partituraSize = 20;
  public notasIndex: DadosNota[] = Array.from({ length: this.partituraSize }, (x, i) => { return {index: i, isBmol: true} });
  public queueNotasIndex: DadosNota[] = [];
  public isScrollerRunning: boolean = false;
  public scrollerController: any;

  constructor(
    private rustInvoker: RustDataSourceService,
    private router: Router,
    private cdRef: ChangeDetectorRef
  ){  }

  ngOnInit(): void {
    this.rustInvoker.connect_midi();
    this.rustInvoker.listen_for_midi_note(this.andUpdateNotes);
  }

  public ngOnDestroy(): void {
    this.rustInvoker.stop_midi();
  }

  public readonly andUpdateNotes = (signal: MidiSignal) => {
    if(signal.state == 128) return;
    this.queueNotasIndex.push(PartituraNotas.notas[signal.note.note]);
    this.cdRef.detectChanges();
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
      this.notasIndex.splice(0,1);
      if(this.queueNotasIndex.length > 0){
        const index = this.queueNotasIndex.splice(0,1);
        this.notasIndex.push(index[0]);
      } else {
        this.notasIndex.push( {index: PartituraNotas.blankNote, isBmol: false} );
      }
    }, 100);
    this.isScrollerRunning = true;
  }

  private stopScroller(){
    clearInterval(this.scrollerController);
    this.isScrollerRunning = false;
  }

}
