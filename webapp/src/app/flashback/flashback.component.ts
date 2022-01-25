import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { MediaPresenterService } from '../media-presenter/media-presenter.service';

type FlashbackDates = string[];
type FlashbackFiles = { [date: string]: FlashbackFile[] };
export type FlashbackFile = { id: number, type: 'IMAGE' | 'VIDEO' };

@Component({
  selector: 'app-flashback',
  templateUrl: './flashback.component.html',
  styleUrls: ['./flashback.component.scss'],
})
export class FlashbackComponent implements OnInit {
  flashbackDates: FlashbackDates = null;
  flashbackFiles: FlashbackFiles = null;

  constructor(
    private http: HttpClient,
    private mediaPresenterService: MediaPresenterService
  ) {
  }

  ngOnInit(): void {
    this.http.get('/api/flashback/dates').subscribe((dateImageIds) => {
      this.flashbackDates = Object.keys(dateImageIds).reverse();
      this.flashbackFiles = dateImageIds as FlashbackFiles;
    });
  }

  hasFlashbacks() {
    return this.flashbackDates !== null && this.flashbackDates.length > 0;
  }

  onImageClick(date: string, file: FlashbackFile) {
    const files = this.flashbackFiles[date];
    const startIndex = files.indexOf(file);
    const items = files.map(file => this.mediaPresenterService.mapToGalleryItem(file.type, file.id));
    this.mediaPresenterService.startPresentation(items, startIndex);
  }
}
