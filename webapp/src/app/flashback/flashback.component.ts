import {Component, OnInit} from '@angular/core';
import {HttpClient} from "@angular/common/http";

type FlashbackDates = string[];
type DateImageIds = { [date: string]: number[]; };

@Component({
  selector: 'app-flashback',
  templateUrl: './flashback.component.html',
  styleUrls: ['./flashback.component.scss']
})
export class FlashbackComponent implements OnInit {

  flashbackDates: FlashbackDates = null;
  dateImageIds: DateImageIds = null;

  constructor(private http: HttpClient) {
  }

  ngOnInit(): void {
    this.http.get('/api/flashback/dates')
      .subscribe(dateImageIds => {
        this.flashbackDates = Object.keys(dateImageIds).reverse();
        this.dateImageIds = dateImageIds as DateImageIds;
      });
  }

  hasNoFlashbacks() {
    return this.flashbackDates !== null && this.flashbackDates.length === 0;
  }
}
