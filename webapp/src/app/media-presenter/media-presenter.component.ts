import {Component, ElementRef, OnInit, ViewChild} from '@angular/core';
import {MediaPresenterService} from "./media-presenter.service";

export interface MediaPresentationSlide {
  imageId: number;

  hasNextSlide: boolean;
  hasPreviousSlide: boolean;

  getNextSlide(): MediaPresentationSlide;
  getPreviousSlide(): MediaPresentationSlide;
}

@Component({
  selector: 'app-media-presenter',
  templateUrl: './media-presenter.component.html',
  styleUrls: ['./media-presenter.component.scss']
})
export class MediaPresenterComponent implements OnInit {

  slide: MediaPresentationSlide | undefined = undefined;

  constructor(
    // private renderer: Renderer2,
    private mediaPresenterService: MediaPresenterService
  ) { }

  ngOnInit(): void {
    this.mediaPresenterService.registerComponent(this);
  }

  setSlide(presentationSlide: MediaPresentationSlide) {
    this.slide = presentationSlide;
  }

  onBackdropClick() {
    this.slide = undefined;
  }

    // this.renderer.addClass(document.body, 'media-presenter-open');
  // this.renderer.removeClass(document.body, 'media-presenter-open');

  imageUrl(): string {
    return `/api/images/${this.slide.imageId}?size=large`;
  }

  gotoPreviousSlide() {
    if (this.slide.hasPreviousSlide) {
      this.slide = this.slide.getPreviousSlide();
    } else {
      this.slide = undefined;
    }
  }
  gotoNextSlide() {
    if (this.slide.hasNextSlide) {
      this.slide = this.slide.getNextSlide();
    } else {
      this.slide = undefined;
    }
  }
}
