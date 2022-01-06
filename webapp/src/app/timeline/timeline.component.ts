import {
  AfterViewChecked,
  ChangeDetectorRef,
  Component,
  OnInit,
  QueryList,
  ViewChildren
} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {DaySectionComponent} from "./day-section/day-section.component";
import {MediaPresenterService} from "../media-presenter/media-presenter.service";
import {TimelineMediaPresentation} from "./timeline-media-presentation";

export type TimelineDates = string[];
export type DateImageIds = { [date: string]: number[]; };

@Component({
  selector: 'app-timeline',
  templateUrl: './timeline.component.html',
  styleUrls: ['./timeline.component.scss']
})
export class TimelineComponent implements OnInit, AfterViewChecked {

  @ViewChildren(DaySectionComponent)
  daySections: QueryList<DaySectionComponent>;

  timelineDates: TimelineDates = null;
  dateImageIds: DateImageIds = null;
  intersectionObserver: IntersectionObserver = null;

  constructor(
    private http: HttpClient,
    private changeDetector: ChangeDetectorRef,
    private mediaPresenterService: MediaPresenterService
  ) { }

  ngOnInit(): void {
    this.http.get('/api/timeline/dates')
      .subscribe(dateImageIds => {
        this.timelineDates = Object.keys(dateImageIds).reverse();
        this.dateImageIds = dateImageIds as DateImageIds;
      });
  }

  ngAfterViewChecked(): void {
    if (this.daySections.length === 0 || this.intersectionObserver !== null) return;

    this.initializeIntersectionObserver();
    this.preloadVisibleDaySections();
    this.changeDetector.detectChanges();
  }

  private initializeIntersectionObserver() {
    const options = {
      rootMargin: '0px',
      threshold: 0.1
    };
    this.intersectionObserver = new IntersectionObserver(entries => {
      entries.forEach(entry => {
        let index = +entries[0].target.getAttribute('data-index');
        let daySection = this.daySections.get(index);
        daySection.setVisible(entry.isIntersecting);
      });
    }, options);

    this.daySections.forEach(daySection => this.intersectionObserver.observe(daySection.elementRef.nativeElement));
  }

  /**
   * Right after initializing the day sections in the template, all day sections are still set to "not visible", also
   * those that are in fact visible. The IntersectionObserver will not trigger for these day sections, since no scroll
   * event occurred. This method will over-approximate and naively mark the fist couple day sections as visible.
   * @private
   */
  private preloadVisibleDaySections() {
    window.scroll(0, 0);
    const windowHeight = window.innerHeight;
    const estimatedNumberOfVisibleSections = Math.ceil(windowHeight / 200);
    for (let i=0; i<estimatedNumberOfVisibleSections; i++) {
      this.daySections.get(i).setVisibleNoDebounce();
    }
  }

  onImageClick(imageId: number) {
    const presentation = new TimelineMediaPresentation(this.timelineDates, this.dateImageIds, imageId);
    this.mediaPresenterService.startPresentation(presentation);
  }
}