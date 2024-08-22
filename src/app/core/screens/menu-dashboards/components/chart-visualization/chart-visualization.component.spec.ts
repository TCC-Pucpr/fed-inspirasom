import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ChartVisualizationComponent } from './chart-visualization.component';

describe('GraphVisualizationComponent', () => {
  let component: ChartVisualizationComponent;
  let fixture: ComponentFixture<ChartVisualizationComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ChartVisualizationComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(ChartVisualizationComponent);
    component = fixture.componentInstance;

    component.chartData = [];
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

});
