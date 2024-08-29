import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MenuGamificadaComponent } from './menu-gamificada.component';

describe('MenuGamificadaComponent', () => {
  let component: MenuGamificadaComponent;
  let fixture: ComponentFixture<MenuGamificadaComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MenuGamificadaComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(MenuGamificadaComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
