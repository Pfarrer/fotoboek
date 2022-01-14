import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { MediaPresenterService } from '../media-presenter/media-presenter.service';
import { GalleryMediaPresentation } from './gallery-media-presentation';

export interface GalleryPath {
  sub_paths: { [name: string]: GalleryPath };
  files: GalleryFile[];
}

export interface GalleryFile {
  id: number;
  file_type: 'IMAGE';
}

@Component({
  selector: 'app-gallery',
  templateUrl: './gallery.component.html',
  styleUrls: ['./gallery.component.scss'],
})
export class GalleryComponent implements OnInit {
  private root_path: GalleryPath = null;
  current_path: GalleryPath = null;

  constructor(
    private http: HttpClient,
    private mediaPresenterService: MediaPresenterService
  ) {}

  ngOnInit(): void {
    this.http
      .get('/api/gallery/paths')
      .subscribe(
        (root_path) =>
          (this.root_path = this.current_path = root_path as GalleryPath)
      );
  }

  preview_images(gallery_path: GalleryPath): number[] {
    if (gallery_path.files.length <= 4) {
      return gallery_path.files.map((file) => file.id);
    }

    const imageIds = [];
    for (let i = 0; i < 4; i++) {
      const fileIndex = Math.floor((gallery_path.files.length / 4) * +i);
      const imageId = gallery_path.files[fileIndex].id;
      imageIds.push(imageId);
    }
    return imageIds;
  }

  onDirectoryClick(gallery_path: GalleryPath) {
    this.current_path = gallery_path;
  }
  onFileClick(file: GalleryFile) {
    const presentation = new GalleryMediaPresentation(this.current_path, file);
    this.mediaPresenterService.startPresentation(presentation);
  }
}
