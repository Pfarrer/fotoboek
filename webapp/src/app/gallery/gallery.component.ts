import {Component, OnInit} from '@angular/core';
import {HttpClient} from "@angular/common/http";

interface GalleryPath {
  sub_paths: { [name: string ]: GalleryPath },
  files: GalleryFile[],
}

interface GalleryFile {
  id: number,
  file_type: 'IMAGE'
}

@Component({
  selector: 'app-gallery',
  templateUrl: './gallery.component.html',
  styleUrls: ['./gallery.component.scss']
})
export class GalleryComponent implements OnInit {

  paths: GalleryPath = null;

  constructor(private http: HttpClient) {
  }

  ngOnInit(): void {
    this.http.get('/api/gallery/paths')
      .subscribe(paths => this.paths = paths as GalleryPath);
  }

}
