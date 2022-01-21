import { Injectable } from '@angular/core';
import { MediaPresenterComponent } from "./media-presenter.component";
import { GalleryItem } from "lightgallery/lg-utils";

@Injectable()
export class MediaPresenterService {
  private component: MediaPresenterComponent;

  registerComponent(component: MediaPresenterComponent) {
    this.component = component;
  }

  startPresentation(items: GalleryItem[], startIndex: number) {
    this.component.startPresentation(items, startIndex);
  }
}
