import { Component, Input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { HousingLocation } from '../housinglocation';
import {RouterModule} from '@angular/router';


@Component({
  selector: 'app-housing-location',
  standalone: true,
  imports: [
    CommonModule,
    RouterModule,
  ],
  template: `
  <section class="listing">
    <img class="listing-photo" [src]="housingLocation.Photo" alt="Exterior photo of {{housingLocation.HouseName}}">
    <h2 class="listing-heading">{{ housingLocation.City }}</h2>
    <p class="listing-location">{{ housingLocation.City }}, {{ housingLocation.StateName }}</p>
    <a [routerLink]="['/details', housingLocation.ID]">Learn More</a>
  </section>
  `,
  styleUrls: ['./housing-location.component.css']
})
export class HousingLocationComponent {
  @Input() housingLocation!: HousingLocation;
}
