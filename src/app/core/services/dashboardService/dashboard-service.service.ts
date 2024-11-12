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

  public async getDashboardData(filter: number): Promise<DailyScoreData[]> {
    switch(filter) {
      case 1:
        this.scoreData = this.rust.getSimpleScoreFromLastWeek();
      break;
      case 2:
        this.scoreData = this.rust.getWeekHighestScores();
      break;
      case 3:
        this.scoreData = this.rust.getTotalBreath();
      break;
      case 4:
        this.scoreData = this.rust.getBreathStr();
      break;
    }
    return this.scoreData;
  }
}
