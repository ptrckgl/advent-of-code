import * as fs from 'fs';

const inputFile = process.argv[2];
const rawData: string = fs.readFileSync(inputFile || 'inputTest.txt', 'utf8');
const data: string[] = rawData.split('\n');

//
// Part 1 + 2
//

const maxRed = 12;
const maxGreen = 13;
const maxBlue = 14;

// Literally go through every single 'game' and check if one of them
// is greater than the total possible amount
let totalIds = 0;
let totalPower = 0;
for (let i = 1; i <= data.length; i++) {
  let strArr = data[i - 1].matchAll(/[0-9]+ [r|g|b]/g);
  let addId = true;
  let maxCurrRed: number = 0,
    maxCurrGreen: number = 0,
    maxCurrBlue: number = 0;
  for (const str of strArr) {
    let splitStr = str.toString().split(' ');
    let num: number = parseInt(splitStr[0]);
    let colour = splitStr[1];
    switch (colour.toString()) {
      case 'r': {
        if (num > maxRed) {
          addId = false;
        }
        maxCurrRed = maxCurrRed == 0 ? num : Math.max(maxCurrRed, num);
        break;
      }
      case 'g': {
        if (num > maxGreen) {
          addId = false;
        }
        maxCurrGreen = maxCurrGreen == 0 ? num : Math.max(maxCurrGreen, num);
        break;
      }
      default: {
        if (num > maxBlue) {
          addId = false;
        }
        maxCurrBlue = maxCurrBlue == 0 ? num : Math.max(maxCurrBlue, num);
        break;
      }
    }
    // if (!addId) break;
  }
  if (addId) totalIds += i;
  totalPower += maxCurrRed * maxCurrGreen * maxCurrBlue;
}

console.log(`Part 1: ${totalIds}`);
console.log(`Part 2: ${totalPower}`);

//
// Part 2
//

// Merged in the top code with part 1 :))
