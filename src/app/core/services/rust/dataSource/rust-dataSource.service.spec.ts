import { TestBed } from '@angular/core/testing';

import { RustDataSourceService } from './rust-dataSource.service';

describe('RustDataSourceService', () => {
  let service: RustDataSourceService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(RustDataSourceService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
