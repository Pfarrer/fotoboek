import {
  Component,
  ElementRef,
  Input,
  OnInit,
  Output,
  EventEmitter,
} from '@angular/core';

@Component({
  selector: 'app-day-section',
  templateUrl: './day-section.component.html',
  styleUrls: ['./day-section.component.scss'],
})
export class DaySectionComponent implements OnInit {
  @Input()
  date: string;

  @Input()
  imageIds: number[];

  @Output()
  imageClick = new EventEmitter<number>();

  isSectionVisible = false;

  constructor(public elementRef: ElementRef) {}

  ngOnInit(): void {}

  setVisible() {
    this.isSectionVisible = true;
  }
}
