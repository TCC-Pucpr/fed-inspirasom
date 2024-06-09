import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PartituraComponent } from './partitura.component';

describe('PartituraComponent', () => {
  let component: PartituraComponent;
  let fixture: ComponentFixture<PartituraComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [PartituraComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(PartituraComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
