import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MenuGamificadoComponent } from './menu-gamificado.component';

describe('MenuGamificadoComponent', () => {
  let component: MenuGamificadoComponent;
  let fixture: ComponentFixture<MenuGamificadoComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MenuGamificadoComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(MenuGamificadoComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
