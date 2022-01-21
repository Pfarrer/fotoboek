import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { ActivatedRoute, Router } from '@angular/router';
import { MediaPresenterService } from '../media-presenter/media-presenter.service';

export interface GalleryPath {
  sub_paths: { [name: string]: GalleryPath };
  files: GalleryFile[];
}

export interface GalleryFile {
  id: number;
  file_name: string;
  file_type: 'IMAGE';
  effective_date: string;
}

@Component({
  selector: 'app-gallery',
  templateUrl: './gallery.component.html',
  styleUrls: ['./gallery.component.scss'],
})
export class GalleryComponent implements OnInit {
  private root_path: GalleryPath = null;
  current_path: GalleryPath = null;
  current_sub_paths: string[] = null;
  recursiveMode = false;

  constructor(
    private http: HttpClient,
    private router: Router,
    private route: ActivatedRoute,
    private mediaPresenterService: MediaPresenterService
  ) {
    this.route.params.subscribe(() =>
      this.update_current_path_by_route_param()
    );
  }

  ngOnInit(): void {
    this.http.get('/api/gallery/paths').subscribe((root_path) => {
      this.root_path = root_path as GalleryPath;
      this.update_current_path_by_route_param();
    });
  }

  get_preview_images_for_sub_path(gallery_path: GalleryPath): number[] {
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

  breadcrumbs(): string[] {
    const path_param = this.route.snapshot.params['path'];
    if (!path_param) {
      return [];
    }
    return path_param.split('/');
  }

  containsSubPaths(): boolean {
    return this.current_path && Object.keys(this.current_path.sub_paths).length > 0;
  }

  getSubPathInfoText(gallery_path: GalleryPath): string {
    const folders_count = Object.keys(gallery_path.sub_paths).length;
    const files_count = gallery_path.files.length;

    let folders_text = '';
    if (folders_count == 1) {
      folders_text = '1 folder';
    } else if (folders_count > 1) {
      folders_text = `${folders_count} folders`;
    }

    let files_text = '';
    if (files_count == 1) {
      files_text = '1 file';
    } else if (files_count > 1) {
      files_text = `${files_count} files`;
    }

    if (folders_text && files_text) {
      return `${folders_text} and ${files_text}`;
    } else {
      return `${folders_text}${files_text}`;
    }
  }

  get_visible_files(): GalleryFile[] {
    function extract_files_recursively(gallery_path: GalleryPath): GalleryFile[] {
      return Object.values(gallery_path.sub_paths).reduce((files, sub_path) => {
        return [...files, ...extract_files_recursively(sub_path)];
      }, gallery_path.files);
    }

    if (!this.recursiveMode) {
      return this.current_path.files;
    } else {
      return extract_files_recursively(this.current_path)
        .sort((a, b) => {
          if (a.effective_date == b.effective_date) {
            return 0;
          } else {
            return a.effective_date > b.effective_date ? 1 : -1;
          }
        });
    }
  }

  onDirectoryClick(sub_path: string) {
    this.router.navigate([
      'gallery',
      {path: this.make_route_path_param(sub_path)},
    ]);
  }

  onBreadcrumbClick(crumb: string) {
    const breadcrumbs = this.breadcrumbs();
    const clicked_index = breadcrumbs.indexOf(crumb);
    const target_path_elems = breadcrumbs.slice(0, clicked_index + 1);
    const target_path = target_path_elems.join('/');
    this.router.navigate(['gallery', {path: target_path}]);
  }

  onFileClick(file: GalleryFile) {
    const files = this.get_visible_files();

    const startIndex = files.indexOf(file);
    const items = files.map((file: GalleryFile) => (
      {
        src: `/api/images/${file.id}?size=large`,
        thumb: `/api/images/${file.id}?size=small`,
        subHtml: file.file_name,
      }
    ));

    this.mediaPresenterService.startPresentation(items, startIndex);
  }

  private make_route_path_param(sub_path: string): string {
    const path_param = this.route.snapshot.params['path'];
    if (path_param) {
      return `${path_param}/${sub_path}`;
    } else {
      return sub_path;
    }
  }

  private update_current_path_by_route_param() {
    this.recursiveMode = false;

    const path_param = this.route.snapshot.params['path'];
    if (!path_param || !this.root_path) {
      this.current_path = this.root_path;
    } else {
      const path_elements = path_param.split('/') as string[];
      this.current_path = path_elements.reduce(
        (gallery_path, path_element) => gallery_path.sub_paths[path_element],
        this.root_path
      );
    }

    this.current_sub_paths = Object.keys(
      (this.current_path || {}).sub_paths || {}
    ).sort();
  }
}
