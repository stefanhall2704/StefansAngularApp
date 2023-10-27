// const sum = require('./sumj');
const housingServer = require('../housing.service')
// import { HousingService } from '../housing.service';
// export interface HousingLocation {
//   ID: number;
//   HouseName: string;
//   City: string;
//   StateName: string;
//   Photo: string;
//   Wifi: boolean;
//   Laundry: boolean;
// }

// test('adds 1 + 2 to equal 3', () => {
//   expect(sum(1, 2)).toBe(3);
// });
test('Getting list of houses', async () => {
  const result = await housingServer.HousingService.getAllHousingLocations();
  console.log(result);
  expect(result).toBe("Testing failure");
})