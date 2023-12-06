import * as fs from 'fs';
import { exit } from 'process';

const inputFile = process.argv[2];
const rawData: string = fs.readFileSync(inputFile || 'inputTest.txt', 'utf8');
const data: string[] = rawData.split('\n');

//
// Part 1
//

let total = 0;
for (let i = 0; i < data.length; i++) {
  let str: string = data[i].replace(/[^0-9]/g, '');
  total += Number(str[0]) * 10 + Number(str.slice(-1));
}
console.log(`Part 1 total: ${total}`);

//
// Part 2
//

const filterArr: Map<string, number> = new Map<string, number>([
  ['one', 1],
  ['two', 2],
  ['three', 3],
  ['four', 4],
  ['five', 5],
  ['six', 6],
  ['seven', 7],
  ['eight', 8],
  ['nine', 9],
  ['1', 1],
  ['2', 2],
  ['3', 3],
  ['4', 4],
  ['5', 5],
  ['6', 6],
  ['7', 7],
  ['8', 8],
  ['9', 9],
]);

let total2 = 0;
for (let i = 0; i < data.length; i++) {
  let str: string = data[i];
  let tempSum = 0;

  // keep increasing idx until we find the first match
  for (let idx = 1; idx <= str.length; idx++) {
    let found = false;
    filterArr.forEach((value, key) => {
      if (str.slice(0, idx).includes(key)) {
        total2 += value * 10;
        found = true;
        return;
      }
    });
    if (found) break;
  }

  // do the same thing but opposite this time - going from the back
  for (let idx = str.length - 1; idx >= 0; idx--) {
    let found = false;
    filterArr.forEach((value, key) => {
      if (str.slice(idx, str.length).includes(key)) {
        total2 += value;
        found = true;
        return;
      }
    });
    if (found) break;
  }
}

console.log(`Part 2 total: ${total2}`);
