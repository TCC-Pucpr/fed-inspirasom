import { ComponentFixture, TestBed } from '@angular/core/testing';

import { GamificadaComponent } from './gamificada.component';

describe('MenuGamificadoComponent', () => {
  let component: GamificadaComponent;
  let fixture: ComponentFixture<GamificadaComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [GamificadaComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(GamificadaComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
