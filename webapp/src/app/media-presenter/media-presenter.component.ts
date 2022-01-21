import { Component, OnInit } from '@angular/core';
import { MediaPresenterService } from './media-presenter.service';
import { LightGallery } from "lightgallery/lightgallery";
import lgZoom from 'lightgallery/plugins/zoom';
import { InitDetail } from "lightgallery/lg-events";
import { GalleryItem } from "lightgallery/lg-utils";

@Component({
  selector: 'app-media-presenter',
  templateUrl: './media-presenter.component.html',
  styleUrls: ['./media-presenter.component.scss'],
})
export class MediaPresenterComponent implements OnInit {
  lightGallery: LightGallery = null;

  settings = {
    counter: false,
    loop: false,
    plugins: [lgZoom]
  };

  onLightGalleryInit = (detail: InitDetail): void => {
    this.lightGallery = detail.instance;
  };

  constructor(
    private mediaPresenterService: MediaPresenterService
  ) {}

  ngOnInit(): void {
    this.mediaPresenterService.registerComponent(this);
  }

  startPresentation(items: GalleryItem[], startIndex: number) {
    this.lightGallery.galleryItems = items;
    this.lightGallery.openGallery(startIndex);
  }
}
