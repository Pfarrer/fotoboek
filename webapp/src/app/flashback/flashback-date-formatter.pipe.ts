import { Pipe, PipeTransform } from '@angular/core';

@Pipe({
  name: 'flashbackDateFormatter',
})
export class FlashbackDateFormatterPipe implements PipeTransform {
  transform(value: string, ...args: unknown[]): unknown {
    const parsed_date = new Date(value);
    const parsed_year = parsed_date.getFullYear();
    const current_year = new Date().getFullYear();
    const difference_years = current_year - parsed_year;
    if (difference_years == 1) {
      return 'Last year';
    } else {
      return `${difference_years} years ago`;
    }
  }
}
