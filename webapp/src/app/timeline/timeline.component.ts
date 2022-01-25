import { ChangeDetectorRef, Component, HostListener, OnInit, QueryList, ViewChildren, } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { DaySectionComponent } from './day-section/day-section.component';
import { MediaPresenterService } from '../media-presenter/media-presenter.service';

export type TimelineDates = string[];
export type TimelineFiles = { [date: string]: TimelineFile[] };
export type TimelineFile = { id: number, type: 'IMAGE' | 'VIDEO' };

declare var M: any;

@Component({
  selector: 'app-timeline',
  templateUrl: './timeline.component.html',
  styleUrls: ['./timeline.component.scss'],
})
export class TimelineComponent implements OnInit {
  @ViewChildren(DaySectionComponent)
  daySections: QueryList<DaySectionComponent>;

  allTimelineDates: TimelineDates = null;
  timelineDates: TimelineDates = null;
  timelineFiles: TimelineFiles = null;
  infiniteScrollManager: InfiniteScrollManager = null;

  scrollSpyInstances: any = null;

  constructor(
    private http: HttpClient,
    private changeDetector: ChangeDetectorRef,
    private mediaPresenterService: MediaPresenterService
  ) {
  }

  ngOnInit(): void {
    this.http.get('/api/timeline/dates').subscribe((dateImageIds) => {
      this.timelineFiles = dateImageIds as TimelineFiles;
      this.allTimelineDates = Object.keys(dateImageIds).reverse();

      const estimatedNumberOfVisibleSections = TimelineComponent.estimatedNumberOfVisibleSections();
      this.infiniteScrollManager = new InfiniteScrollManager(
        estimatedNumberOfVisibleSections,
        3*estimatedNumberOfVisibleSections,
        this.allTimelineDates
      );
      this.timelineDates = this.infiniteScrollManager.moveTo(this.allTimelineDates[0]);
      this.updateScrollspy();
    });
  }

  @HostListener('window:scroll', [])
  onScroll() {
    const bufferY = 100;

    // Check if top is reached
    if (window.scrollY < bufferY) {
      this.timelineDates = this.infiniteScrollManager.extendTop(2);
      this.updateScrollspy();
    }

    // Check if bottom is reached
    if ((window.innerHeight + window.scrollY + bufferY) >= document.body.offsetHeight) {
      this.timelineDates = this.infiniteScrollManager.extendBottom(2);
      this.updateScrollspy();
    }
  }

  private static estimatedNumberOfVisibleSections(): number {
    const windowHeight = window.innerHeight;
    return Math.ceil(windowHeight / 100);
  }

  onImageClick(file: TimelineFile) {
    const files = this.timelineDates.reduce((arr, date) => {
      return [
        ...arr,
        ...this.timelineFiles[date]
      ]
    }, [] as TimelineFile[]);

    const items = files.map(file => this.mediaPresenterService.mapToGalleryItem(
      file.type, file.id
    ));
    const startIndex = files.indexOf(file);
    this.mediaPresenterService.startPresentation(items, startIndex);
  }

  private updateScrollspy() {
    if (this.scrollSpyInstances) {
      this.scrollSpyInstances.forEach(instance => instance.destroy());
    }
    const scrollspyElements = document.querySelectorAll('.scrollspy');
    this.scrollSpyInstances = M.ScrollSpy.init(scrollspyElements, {});
  }

  onScrollspyClick(date: string) {
    this.timelineDates = this.infiniteScrollManager.moveTo(date);
  }
}

class InfiniteScrollManager {

  private visibleRangeTop = 0;
  private visibleRangeBottom = 0;

  constructor(
    private minRange: number,
    private maxRange: number,
    private dates: string[]
  ) {
    this.visibleRangeBottom = Math.min(minRange, this.dates.length-1);
  }

  extendTop(count: number) {
    this.visibleRangeTop = Math.max(0, this.visibleRangeTop - count);
    if (this.visibleRangeBottom - this.visibleRangeTop > this.maxRange) {
      this.visibleRangeBottom = this.visibleRangeTop + this.maxRange;
    }
    return this.dates.slice(this.visibleRangeTop, this.visibleRangeBottom);
  }

  extendBottom(count: number) {
    this.visibleRangeBottom = Math.min(this.dates.length - 1, this.visibleRangeBottom + count);
    if (this.visibleRangeBottom - this.visibleRangeTop > this.maxRange) {
      this.visibleRangeTop = this.visibleRangeBottom - this.maxRange;
    }
    return this.dates.slice(this.visibleRangeTop, this.visibleRangeBottom);
  }

  moveTo(date: string) {
    this.visibleRangeTop = this.dates.indexOf(date);
    this.visibleRangeBottom = Math.min(this.dates.length - 1, this.visibleRangeTop + this.minRange);
    return this.dates.slice(this.visibleRangeTop, this.visibleRangeBottom);
  }
}
