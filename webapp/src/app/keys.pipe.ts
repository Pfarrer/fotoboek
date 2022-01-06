import { Pipe, PipeTransform } from '@angular/core';

@Pipe({
  name: 'keys'
})
export class KeysPipe implements PipeTransform {

  transform(value: Object, ...args: unknown[]): unknown {
    return Object.keys(value);
  }

}
