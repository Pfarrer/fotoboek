import { Injectable } from '@angular/core';
import { MediaPresenterComponent } from "./media-presenter.component";
import { GalleryItem } from "lightgallery/lg-utils";
import { VideoInfo } from "lightgallery/types";

@Injectable()
export class MediaPresenterService {
  private component: MediaPresenterComponent;

  registerComponent(component: MediaPresenterComponent) {
    this.component = component;
  }

  startPresentation(items: GalleryItem[], startIndex: number) {
    this.component.startPresentation(items, startIndex);
  }

  mapToGalleryItem(
    file_type: 'IMAGE' | 'VIDEO',
    file_id: number
  ): GalleryItem {
    if (file_type === 'IMAGE') {
      return {
        src: `/api/images/${file_id}?size=large`,
        thumb: `/api/images/${file_id}?size=small`,
      };
    } else if (file_type === 'VIDEO') {
      const attributes: HTMLVideoElement = {
        preload: false,
        controls: true
      } as unknown as HTMLVideoElement;
      return {
        video: {
          source: [
            { src: `/api/videos/${file_id}`, type: 'video/webm' }
          ],
          tracks: [],
          attributes,
        }
      } as VideoInfo;
    } else {
      console.error(`Unsupported file type: ${file_type}`);
    }
  }
}
