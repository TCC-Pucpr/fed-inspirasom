import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { ChartVisualizationComponent } from "./components/chart-visualization/chart-visualization.component";
import { GraphData } from '../../model/GraphData.model';
import { NumericVisualizationComponent } from "./components/numeric-visualization/numeric-visualization.component";
import { SidebarComponent } from "../components/sidebar/sidebar.component";
import { SidebarService } from '../../services/sidebar-service/sidebar.service';
import { ButtonModule } from 'primeng/button';
import { ListVisualizationComponent } from "./components/list-visualization/list-visualization.component";

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
  public testData: GraphData[] = [
    { date: "01/01", score: 10 },
    { date: "01/02", score: 9.5 },
    { date: "01/03", score: 9 },
    { date: "01/04", score: 8.5 },
    { date: "01/05", score: 8 },
    { date: "01/06", score: 7.5 },
    { date: "01/07", score: 7 },
  ];

  constructor(
    protected sidebarService: SidebarService
  ) { }

}
