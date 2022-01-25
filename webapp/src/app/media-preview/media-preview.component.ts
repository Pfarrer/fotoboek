import { Component, Input, OnInit } from '@angular/core';

@Component({
  selector: 'app-media-preview',
  templateUrl: './media-preview.component.html',
  styleUrls: ['./media-preview.component.scss']
})
export class MediaPreviewComponent implements OnInit {

  @Input()
  file_id: number;

  @Input()
  file_type: 'IMAGE' | 'VIDEO';

  constructor() { }

  ngOnInit(): void {
  }

}
