import { TestBed } from '@angular/core/testing';

import { RustService } from './rust.service';

describe('RustService', () => {
  let service: RustService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(RustService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
