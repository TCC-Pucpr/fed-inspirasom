import { Component } from '@angular/core';
import { ChartVisualizationComponent } from "./components/chart-visualization/chart-visualization.component";
import { NumericVisualizationComponent } from "./components/numeric-visualization/numeric-visualization.component";
import { SidebarComponent } from "../components/sidebar/sidebar.component";
import { SidebarService } from '../../services/sidebarService/sidebar.service';
import { ButtonModule } from 'primeng/button';
import { ListVisualizationComponent } from "./components/list-visualization/list-visualization.component";
import { RustService } from '../../services/rust/rust.service';
import { DailyScoreData } from '../../model/DailyScoreData';

@Component({
  selector: 'app-menu-dashboards',
  standalone: true,
  imports: [
    ChartVisualizationComponent,
    NumericVisualizationComponent,
    SidebarComponent,
    ButtonModule,
    ListVisualizationComponent
],
  templateUrl: './menu-dashboards.component.html',
  styleUrl: './menu-dashboards.component.scss'
})
export class MenuDashboardsComponent {

  public sidebarVisible: boolean = false;
  public scoreDataPromise: Promise<DailyScoreData[]>;
  public scoreData: DailyScoreData[];
  public daysInArow: number = 0;

  constructor(
    protected sidebarService: SidebarService,
    protected rust: RustService,
  ) { }

  public async ngOnInit() {
    this.daysInArow = await this.rust.getConsecutiveDays();
  }

}
