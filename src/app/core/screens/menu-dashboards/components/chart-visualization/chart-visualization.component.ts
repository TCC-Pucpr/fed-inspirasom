import { Component, Inject, Input, OnInit, ViewChild } from '@angular/core';
import { ChartModule } from 'primeng/chart';
import { GraphData as ChartData } from '../../../../model/GraphData.model';
import { CommonModule, DOCUMENT } from '@angular/common';
import { ThemeService } from '../../../../services/themeService/theme.service';
@Component({
  selector: 'app-chart-visualization',
  standalone: true,
  imports: [ 
    CommonModule, 
    ChartModule
  ],
  templateUrl: './chart-visualization.component.html',
  styleUrl: './chart-visualization.component.scss'
})
export class ChartVisualizationComponent implements OnInit {

  @Input() chartData: ChartData[];

  public refreshChart: boolean = false;
  public dates: string[] = [];
  public scores: number[] = [];

  public data: any;
  public options: any;

  constructor(
    private themeService: ThemeService,
    @Inject(DOCUMENT) private document: Document,
  ) {
  }

  public ngOnInit(): void {
    for(let data of this.chartData){
      this.dates.push(data.date);
      this.scores.push(data.score);
    }
    this.buildChart();
    setTimeout(() => {
      this.updateChart();
    }, 1);
  }

  public ngAfterViewInit(): void {
    this.themeService.onThemeChange.subscribe(() => { this.updateChart() });
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

  public updateChart() {
    this.refreshChart = true;
    setTimeout(() => {
      this.buildChart();
      this.refreshChart = false;  
    }, 1);
  }

  protected buildChart() {
    const documentStyle = getComputedStyle(this.document.documentElement);
    const textColor = documentStyle.getPropertyValue('--primary-color-text');
    const borderColor = documentStyle.getPropertyValue('--secondary-color');
    const gridColor = documentStyle.getPropertyValue('--primary-300');
    
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
  }
  
}
