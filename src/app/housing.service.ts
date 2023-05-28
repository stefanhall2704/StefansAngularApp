import { Injectable } from '@angular/core';
import { HousingLocation } from './housinglocation';

@Injectable({
  providedIn: 'root'
})
export class HousingService {
  url = 'http://localhost:3000/locations';
  async getAllHousingLocations(): Promise<HousingLocation[]> {
    const data = await fetch(this.url);
    return await data.json() ?? [];
  }
  async getHousingLocationById(id: number): Promise<HousingLocation | undefined> {
    const data = await fetch(`${this.url}/${id}`);
    return await data.json() ?? {};
  }
  constructor() { }
  submitApplication(houseName: string, cityName: string, stateName: string, photo: string, wifi: string, laundry: string) {
    const data = {
      houseName: houseName,
      cityName: cityName,
      stateName: stateName,
      photo: photo,
      wifi: wifi,
      laundry: laundry,
    };
    console.log(data);
  }
}

