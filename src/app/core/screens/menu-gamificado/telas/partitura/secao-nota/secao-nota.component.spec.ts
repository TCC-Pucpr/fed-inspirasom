import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SecaoNotaComponent } from './secao-nota.component';

describe('SecaoNotaComponent', () => {
  let component: SecaoNotaComponent;
  let fixture: ComponentFixture<SecaoNotaComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SecaoNotaComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(SecaoNotaComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
