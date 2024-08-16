import { Component, Input, OnInit } from '@angular/core';
import { ChartModule } from 'primeng/chart';
import { GraphData } from '../../../../model/GraphData.model';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-graph-visualization',
  standalone: true,
  imports: [ CommonModule, ChartModule ],
  templateUrl: './graph-visualization.component.html',
  styleUrl: './graph-visualization.component.scss'
})
export class GraphVisualizationComponent implements OnInit {

  @Input() graphData: GraphData[];
  public dates: string[] = [];
  public scores: number[] = [];

  protected refreshGraph: boolean = false;

  public data: any;
  public options: any;

  constructor() {
  }

  public ngOnInit(): void {
    for(let data of this.graphData){
      this.dates.push(data.date);
      this.scores.push(data.score);
    }
    this.buildGraph();
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

  protected buildGraph() {
    const documentStyle = getComputedStyle(document.documentElement);
    const textColor = documentStyle.getPropertyValue('--primary-color-text');
    const gridColor = documentStyle.getPropertyValue('--primary-400');
    const borderColor = documentStyle.getPropertyValue('--primary-800');

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
