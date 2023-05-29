import { Injectable } from '@angular/core';
import { HousingLocation } from './housinglocation';


interface HouseData {
  houseName: string,
  cityName: string,
  stateName: string,
  photo: string,
  wifi: boolean,
  laundry: boolean,
}

async function postData(url: string = "", data: HouseData) {
  // Default options are marked with *
  const response = await fetch(url, {
    method: "POST", // *GET, POST, PUT, DELETE, etc.
    mode: "no-cors", // no-cors, *cors, same-origin
    cache: "no-cache", // *default, no-cache, reload, force-cache, only-if-cached
    credentials: "same-origin", // include, *same-origin, omit
    headers: {
      "Content-Type": "application/json",
    },
    redirect: "follow", // manual, *follow, error
    referrerPolicy: "no-referrer", // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
    body: JSON.stringify(data), // body data type must match "Content-Type" header
  });
  return response.json(); // parses JSON response into native JavaScript objects
}

@Injectable({
  providedIn: 'root'
})
export class HousingService {
  url = 'http://localhost:8000/api/all_listings';
  async getAllHousingLocations(): Promise<HousingLocation[]> {
    const response = await fetch(this.url);
    const data = await response.json();
    console.log(data);
    return data ? data : [];
  }
  
  
  async getHousingLocationById(id: number): Promise<HousingLocation | undefined> {
    const response = await fetch(`http://localhost:8000/api/listing/${id}`);
    const data = await response.json();
    console.log(data);
    return data ? data : [];
  }
  constructor() { }
  async submitApplication(houseName: string, cityName: string, stateName: string, photo: string, wifi: string, laundry: string) {
    localStorage.setItem("createListing", `Listing for ${houseName} has successfully been listed`);
    const data: HouseData = {
      houseName: houseName,
      cityName: cityName,
      stateName: stateName,
      photo: photo,
      wifi: Boolean(wifi),
      laundry: Boolean(laundry),
    };

    let url: string = "http://localhost:8000/api/house/create";
    await postData(url, data)
    .then((data) => {

    })
    .finally(() => {
      window.location.href = '/';
    });
    
    console.log(data);
  }
}

