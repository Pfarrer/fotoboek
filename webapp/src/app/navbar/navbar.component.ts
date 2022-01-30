import {AfterViewInit, Component, OnInit} from '@angular/core';

declare var M: any;

@Component({
  selector: 'app-navbar',
  templateUrl: './navbar.component.html',
  styleUrls: ['./navbar.component.scss']
})
export class NavbarComponent implements OnInit, AfterViewInit {

  navLinks = [
    { url: '/timeline', icon: 'view_timeline', text: 'Timeline' },
    { url: '/gallery', icon: 'perm_media', text: 'Gallery' },
    { url: '/flashback', icon: 'psychology', text: 'Flashback' },
    { url: '/people', icon: 'people', text: 'People' },
    { url: '/geo', icon: 'public', text: 'Geographic' },
    { url: '/admin', icon: 'insights', text: 'Admin' },
  ];

  constructor() { }

  ngOnInit(): void {
  }

  ngAfterViewInit(): void {
    const navSlideOutElements = document.querySelectorAll('app-navbar #nav-slide-out');
    M.Sidenav.init(navSlideOutElements, {});
  }

}
