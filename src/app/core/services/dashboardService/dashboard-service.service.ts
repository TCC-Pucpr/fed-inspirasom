import { Injectable } from '@angular/core';
import { RustService } from '../rust/rust.service';
import { DailyScoreData } from '../../model/DailyScoreData';

@Injectable({
  providedIn: 'root'
})
export class DashboardServiceService {

  private scoreData: Promise<DailyScoreData[]>;

  constructor(
    private rust: RustService
  ) { }

  public async getDashboardData(): Promise<DailyScoreData[]> {
    this.scoreData = this.rust.getSimpleScoreFromLastWeek();
    return this.scoreData;
  }
}
