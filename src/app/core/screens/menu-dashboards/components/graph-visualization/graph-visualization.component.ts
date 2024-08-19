import { Component, Input, OnInit, ViewChild } from '@angular/core';
import { ChartModule, UIChart } from 'primeng/chart';
import { GraphData } from '../../../../model/GraphData.model';
import { CommonModule } from '@angular/common';
import { ThemeService } from '../../../../services/theme-service/theme.service';
@Component({
  selector: 'app-graph-visualization',
  standalone: true,
  imports: [ 
    CommonModule, 
    ChartModule
  ],
  templateUrl: './graph-visualization.component.html',
  styleUrl: './graph-visualization.component.scss'
})
export class GraphVisualizationComponent implements OnInit {

  @Input() graphData: GraphData[];
  @ViewChild(UIChart) chart: UIChart;
  public dates: string[] = [];
  public scores: number[] = [];

  protected refreshGraph: boolean = false;

  public data: any;
  public options: any;

  constructor(
    private themeService: ThemeService,
  ) {
  }

  public ngOnInit(): void {
    for(let data of this.graphData){
      this.dates.push(data.date);
      this.scores.push(data.score);
    }
    this.buildGraph();
  }

  public ngAfterViewInit(): void {
    this.themeService.onThemeChange.subscribe(this.buildGraph);
  }

  //honestamente eu não sei definir a tipagem desses caras, a lib foi feita em js
  public test(param1: any, param2: any[], param3: any) {
    if(param2.length > 0){
      console.log('cliquei em um ponto!');
      console.log(param2);
    }
  }

  public onEdit(){

  }

  public updateGraph() {
    this.chart.refresh();
  }

  protected buildGraph() {
    // TODO ajeitar o grafico pra ele ser re-renderizado quando o tema for mudado
    // TODO ajeitar o primeiro render do grafico, ele ta renderizando errado
    const documentStyle = getComputedStyle(document.documentElement);
    const textColor = documentStyle.getPropertyValue('--primary-color-text');
    const borderColor = documentStyle.getPropertyValue('--primary-color-text');
    const gridColor = documentStyle.getPropertyValue('--primary-color-text');
    
    const labels = this.dates;
    const data = this.scores;

    this.data = {
      labels,
      datasets: [
        {
          label: 'Progresso',
          data,
          fill: false,
          borderColor: borderColor,
          tension: 0.1,
        }
      ],
    };

    this.options = {
      maintainAspectRatio: false,
      aspectRatio: 0.6,
      plugins: {
        legend: {
          labels: {
            color: textColor,
          },
        },
      },
      scales: {
        x: {
          ticks: {
            color: textColor,
          },
          grid: {
            color: gridColor,
            drawBorder: true,
          },
        },
        y: {
          ticks: {
            color: textColor,
          },
          grid: {
            color: gridColor,
            drawBorder: false,
          },
        },
      },
      onClick: this.test
    };
    if(this.chart){
      this.chart.refresh();
    }
  }
  
}
