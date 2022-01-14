import { MediaPresentationSlide } from '../media-presenter/media-presenter.component';
import { GalleryPath, GalleryFile } from './gallery.component';

export class GalleryMediaPresentation implements MediaPresentationSlide {
  private currentIndex: number;

  constructor(private gallery_path: GalleryPath, private file: GalleryFile) {
    this.currentIndex = gallery_path.files.indexOf(this.file);
  }

  get imageId(): number {
    return this.gallery_path.files[this.currentIndex].id;
  }

  get hasNextSlide(): boolean {
    return this.gallery_path.files.length - 1 > this.currentIndex;
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
