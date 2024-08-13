import { ComponentFixture, TestBed } from '@angular/core/testing';

import { NumericVisualizationComponent } from './numeric-visualization.component';

describe('NumericVisualizationComponent', () => {
  let component: NumericVisualizationComponent;
  let fixture: ComponentFixture<NumericVisualizationComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [NumericVisualizationComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(NumericVisualizationComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
