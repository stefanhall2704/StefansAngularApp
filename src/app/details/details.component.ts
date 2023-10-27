import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ActivatedRoute, RouterLink } from '@angular/router';
import { HousingService } from '../housing.service';
import { HousingLocation } from '../housinglocation';
import { FormControl, FormGroup, ReactiveFormsModule } from '@angular/forms';




@Component({
  selector: 'app-details',
  standalone: true,
  imports: [
    CommonModule,
    ReactiveFormsModule,
  ],
  template: `
  <article>
    <img class="listing-photo" [src]="housingLocation?.Photo"
      alt="Exterior photo of {{housingLocation?.HouseName}}"/>
    <section class="listing-description">
      <form [formGroup]="applyForm" (submit)="deleteListing()">
      <button type="submit" class="primary">Delete Listing</button>
      </form>
      <h2 class="listing-heading">{{housingLocation?.HouseName}}</h2>
      <p class="listing-location">{{housingLocation?.City}}, {{housingLocation?.StateName}}</p>
    </section>
    <section class="listing-features">
      <h2 class="section-heading">About this housing location</h2>
      <ul>
        <li>Wifi: {{ housingLocation?.Wifi ? 'Yes' : 'No' }}</li>
        <li>Washer/Dryer: {{housingLocation?.Laundry ? 'Yes' : 'No'}}</li>
      </ul>
    </section>
  </article>
`,
  styleUrls: ['./details.component.css']
})
export class DetailsComponent {

  route: ActivatedRoute = inject(ActivatedRoute);
  housingService = inject(HousingService);
  housingLocation: HousingLocation | undefined;
  applyForm = new FormGroup({
    ID: new FormControl(''),
  });
  constructor() {
    const housingLocationId = parseInt(this.route.snapshot.params['id'], 10);
    this.housingService.getHousingLocationById(housingLocationId).then(housingLocation => {
      this.housingLocation = housingLocation;
    });
  }
  deleteListing() {
    this.housingService.deleteListing()
  }
}
