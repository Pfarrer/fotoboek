import { MediaPresentationSlide } from '../media-presenter/media-presenter.component';

export class FlashbackMediaPresentation implements MediaPresentationSlide {
  private currentIndex: number;

  constructor(private imageIds: number[], startImageId: number) {
    this.currentIndex = imageIds.indexOf(startImageId);
  }

  get imageId(): number {
    return this.imageIds[this.currentIndex];
  }

  get hasNextSlide(): boolean {
    return this.imageIds.length - 1 > this.currentIndex;
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
