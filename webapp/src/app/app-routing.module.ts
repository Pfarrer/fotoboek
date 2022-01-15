import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { GalleryComponent } from './gallery/gallery.component';
import { TimelineComponent } from './timeline/timeline.component';
import { FlashbackComponent } from './flashback/flashback.component';
import { PeopleComponent } from './people/people.component';
import { GeoComponent } from './geo/geo.component';
import { StatsComponent } from './stats/stats.component';

const routes: Routes = [
  { path: '', pathMatch: 'full', redirectTo: 'timeline' },
  { path: 'timeline', component: TimelineComponent },
  { path: 'gallery', component: GalleryComponent },
  { path: 'gallery/:path', component: GalleryComponent },
  { path: 'flashback', component: FlashbackComponent },
  { path: 'people', component: PeopleComponent },
  { path: 'geo', component: GeoComponent },
  { path: 'stats', component: StatsComponent },
];

@NgModule({
  imports: [RouterModule.forRoot(routes, { useHash: true })],
  exports: [RouterModule],
})
export class AppRoutingModule {}
