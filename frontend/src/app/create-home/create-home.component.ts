import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ActivatedRoute, RouterLink } from '@angular/router';
import { HousingService } from '../housing.service';
import { HousingLocation } from '../housinglocation';
import { FormControl, FormGroup, ReactiveFormsModule } from '@angular/forms';

@Component({
  selector: 'app-create-home',
  standalone: true,
  imports: [
    CommonModule,
    ReactiveFormsModule,
  ],
  template: `
  <article>
    <section class="listing-apply">
      <h2 class="section-heading">Create Home Listing</h2>
      <form [formGroup]="applyForm" (submit)="submitApplication()">
        <label for="house-name">House Name</label>
        <input id="house-name" type="text" formControlName="houseName">

        <label for="city-name">City</label>
        <input id="city-name" type="text" formControlName="cityName">

        <label for="state-name">State</label>
        <input id="state-name" type="text" formControlName="stateName">

        <label for="photo">Photo</label>
        <input id="photo" type="text" placeholder="Paste link to photo..." formControlName="photo">

        <label for="wifi" name="wifi">Wifi</label>
        <div class="select-container">
          <select class="select-container" for="wifi_name" formControlName="wifi">
            <option value="true">Yes</option>
            <option value="false">No</option>
          </select>
        </div>
        <br><br>
        <label for="laundry" name="laundry">Washer/Dryer Included</label>
        <div class="select-container">
          <select for="laundry_name" formControlName="laundry">
            <option value="true">Yes</option>
            <option value="false">No</option>
          </select>
        </div>
<br><br>
        <button type="submit" class="primary">Create Listing</button>
      </form>
    </section>
  </article>
  `,
  styleUrls: ['./create-home.component.css']
})
export class CreateHomeComponent {
  route: ActivatedRoute = inject(ActivatedRoute);
  housingService = inject(HousingService);
  housingLocation: HousingLocation | undefined;
  applyForm = new FormGroup({
    houseName: new FormControl(''),
    cityName: new FormControl(''),
    stateName: new FormControl(''),
    photo: new FormControl(''),
    wifi: new FormControl(''),
    laundry: new FormControl('')
  });
  constructor() {
    const housingLocationId = parseInt(this.route.snapshot.params['id'], 10);
    this.housingService.getHousingLocationById(housingLocationId).then(housingLocation => {
      this.housingLocation = housingLocation;
    });
  }
  submitApplication() {
    this.housingService.submitApplication(
      this.applyForm.value.houseName ?? '',
      this.applyForm.value.cityName ?? '',
      this.applyForm.value.stateName ?? '',
      this.applyForm.value.photo ?? '',
      this.applyForm.value.wifi ?? 'false',
      this.applyForm.value.laundry ?? 'false',
    )
  }
}
