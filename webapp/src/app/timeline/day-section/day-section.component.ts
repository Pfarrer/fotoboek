import {
  Component,
  ElementRef,
  Input,
  OnInit, Output,
  EventEmitter
} from '@angular/core';
import {Subject} from "rxjs";
import {debounceTime} from "rxjs/operators";

@Component({
  selector: 'app-day-section',
  templateUrl: './day-section.component.html',
  styleUrls: ['./day-section.component.scss']
})
export class DaySectionComponent implements OnInit {

  @Input()
  date: string;

  @Input()
  imageIds: number[];

  @Output()
  imageClick = new EventEmitter<number>();

  isSectionVisible = false;
  private isSectionVisibleDebounce$ = new Subject<boolean>();

  constructor(public elementRef: ElementRef) { }

  ngOnInit(): void {
    this.isSectionVisibleDebounce$.pipe(debounceTime(100)).subscribe(isVisible => {
      if (isVisible) {
        this.isSectionVisible = true;
        this.isSectionVisibleDebounce$.complete();
      }
    })
  }

  setVisible(isVisible: boolean) {
    if (!this.isSectionVisible) this.isSectionVisibleDebounce$.next(isVisible);
  }

  setVisibleNoDebounce() {
    this.isSectionVisible = true;
    this.isSectionVisibleDebounce$.complete();
  }
}
