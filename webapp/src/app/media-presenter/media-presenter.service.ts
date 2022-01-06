import {Injectable} from '@angular/core';
import {MediaPresentationSlide, MediaPresenterComponent} from "./media-presenter.component";

@Injectable()
export class MediaPresenterService  {

  private component: MediaPresenterComponent;

  registerComponent(component: MediaPresenterComponent) {
    this.component = component;
  }

  startPresentation(presentationSlide: MediaPresentationSlide) {
    this.component.setSlide(presentationSlide);
  }

}
