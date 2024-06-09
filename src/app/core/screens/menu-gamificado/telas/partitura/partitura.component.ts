import { Component, OnDestroy, OnInit } from '@angular/core';
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

  public notasIndex: number[] = [0,1,2];

  constructor(
    private rustInvoker: RustDataSourceService,
    private router: Router
  ){  }

  ngOnInit(): void {
    this.rustInvoker.connect_midi();
    this.rustInvoker.listen_for_midi_note(this.andGetMidiNote);
  }

  ngOnDestroy(): void {
    this.rustInvoker.stop_midi();
  }

  public voltaMenu() {
    this.router.navigate(['menu-gamificado']);
  }

  public andGetMidiNote(signal: MidiSignal){
    this.notasIndex.push(PartituraNotas.notas[signal.note])
  }

}
