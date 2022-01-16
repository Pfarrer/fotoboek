import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { MediaPresenterService } from '../media-presenter/media-presenter.service';
import { FlashbackMediaPresentation } from './flashback-media-presentation';

type FlashbackDates = string[];
type DateImageIds = { [date: string]: number[] };

@Component({
  selector: 'app-flashback',
  templateUrl: './flashback.component.html',
  styleUrls: ['./flashback.component.scss'],
})
export class FlashbackComponent implements OnInit {
  flashbackDates: FlashbackDates = null;
  dateImageIds: DateImageIds = null;

  constructor(
    private http: HttpClient,
    private mediaPresenterService: MediaPresenterService
  ) {}

  ngOnInit(): void {
    this.http.get('/api/flashback/dates').subscribe((dateImageIds) => {
      this.flashbackDates = Object.keys(dateImageIds).reverse();
      this.dateImageIds = dateImageIds as DateImageIds;
    });
  }

  hasFlashbacks() {
    return this.flashbackDates !== null && this.flashbackDates.length > 0;
  }

  onImageClick(date: string, imageId: number) {
    const presentation = new FlashbackMediaPresentation(
      this.dateImageIds[date],
      imageId
    );
    this.mediaPresenterService.startPresentation(presentation);
  }
}
