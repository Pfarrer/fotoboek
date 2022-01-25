import {
  Component,
  ElementRef,
  Input,
  OnInit,
  Output,
  EventEmitter,
} from '@angular/core';
import { TimelineFile } from "../timeline.component";

@Component({
  selector: 'app-day-section',
  templateUrl: './day-section.component.html',
  styleUrls: ['./day-section.component.scss'],
})
export class DaySectionComponent implements OnInit {
  @Input()
  date: string;

  @Input()
  files: TimelineFile[];

  @Output()
  imageClick = new EventEmitter<TimelineFile>();

  constructor(public elementRef: ElementRef) {}

  ngOnInit(): void {}
}
