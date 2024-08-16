import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { GraphVisualizationComponent } from "./components/graph-visualization/graph-visualization.component";
import { GraphData } from '../../model/GraphData.model';
import { NumericVisualizationComponent } from "./components/numeric-visualization/numeric-visualization.component";

@Component({
  selector: 'app-menu-dashboards',
  standalone: true,
  imports: [GraphVisualizationComponent, NumericVisualizationComponent],
  templateUrl: './menu-dashboards.component.html',
  styleUrl: './menu-dashboards.component.scss'
})
export class MenuDashboardsComponent {

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
    private router: Router
  ) { }

  public redirectGame(): void {
    this.router.navigate(['gamificada']);
  }

}
