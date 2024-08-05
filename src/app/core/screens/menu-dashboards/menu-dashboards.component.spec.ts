import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MenuDashboardsComponent } from './menu-dashboards.component';

describe('MenuDashboardsComponent', () => {
  let component: MenuDashboardsComponent;
  let fixture: ComponentFixture<MenuDashboardsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MenuDashboardsComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(MenuDashboardsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
