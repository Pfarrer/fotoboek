import {MediaPresentationSlide} from "../media-presenter/media-presenter.component";
import {DateImageIds, TimelineDates} from "./timeline.component";

export class TimelineMediaPresentation implements MediaPresentationSlide {

  private readonly allImageIds: number[];
  private currentIndex: number;

  constructor(
    timelineDates: TimelineDates,
    dateImageIds: DateImageIds,
    imageId: number
  ) {
    this.allImageIds = timelineDates.reduce((arr, date) => {
      return [
        ...arr,
        ...dateImageIds[date]
      ]
    }, []);
    this.currentIndex = this.allImageIds.indexOf(imageId);
  }

  get imageId(): number {
    return this.allImageIds[this.currentIndex];
  }

  get hasNextSlide(): boolean {
    return this.allImageIds.length - 1 > this.currentIndex;
  }
  get hasPreviousSlide(): boolean {
    return this.currentIndex > 0;
  }

  getNextSlide(): MediaPresentationSlide {
    if (this.hasNextSlide) {
      this.currentIndex++;
      return this;
    } else {
      return null;
    }
  }

  getPreviousSlide(): MediaPresentationSlide {
    if (this.hasPreviousSlide) {
      this.currentIndex--;
      return this;
    } else {
      return null;
    }
  }

}
