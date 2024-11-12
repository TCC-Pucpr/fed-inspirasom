import { Component, ViewChild } from '@angular/core';
import { ChartVisualizationComponent } from "./components/chart-visualization/chart-visualization.component";
import { NumericVisualizationComponent } from "./components/numeric-visualization/numeric-visualization.component";
import { SidebarComponent } from "../components/sidebar/sidebar.component";
import { SidebarService } from '../../services/sidebarService/sidebar.service';
import { ButtonModule } from 'primeng/button';
import { RustService } from '../../services/rust/rust.service';
import { DailyScoreData } from '../../model/DailyScoreData';
import { DropdownModule } from 'primeng/dropdown';
import { FormsModule } from '@angular/forms';
import { DadosDropdown } from '../../model/GraphData.model';

@Component({
  selector: 'app-menu-dashboards',
  standalone: true,
  imports: [
    FormsModule,
    ChartVisualizationComponent,
    NumericVisualizationComponent,
    SidebarComponent,
    ButtonModule,
    DropdownModule
  ],
  templateUrl: './menu-dashboards.component.html',
  styleUrl: './menu-dashboards.component.scss'
})
export class MenuDashboardsComponent {

  @ViewChild('dash') dash: ChartVisualizationComponent;

  public sidebarVisible: boolean = false;
  public scoreDataPromise: Promise<DailyScoreData[]>;
  public scoreData: DailyScoreData[];
  public daysInArow: number = 0;

  public dropdownOptions: DadosDropdown[] = [
    { titulo: "Média scores", id: 1 },
    { titulo: "Média maior sequência de acertos", id: 2 },
    { titulo: "Média da duração total do sopro", id: 3 },
    { titulo: "Média força de sopro", id: 4 }
  ]
  public selectedOption: DadosDropdown = this.dropdownOptions[0];

  constructor(
    protected sidebarService: SidebarService,
    protected rust: RustService,
  ) { }

  public async ngOnInit() {
    this.daysInArow = await this.rust.getConsecutiveDays();
  }

  public async onDropdownSelect(data: DadosDropdown) {
    this.selectedOption = data;
    this.dash.setFilter(data.id);
  }


}
