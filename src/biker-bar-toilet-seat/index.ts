import { open } from "node:fs/promises";
import fs from "node:fs";

import { dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const path = `${__dirname}/lines`;

const handle = await open(path);

for await (const line of handle.readLines()) {
  console.log(line);
}

console.log("\n");

// Straightforward way to read the file and split it into lines

fs.readFileSync(path)
  .toString()
  .split("\n")
  .forEach((line) => console.log(line));

// Track and then print how long it takes to open file and store every other line 1000000 times
let temp = "";
// Will take on the order of ~20 seconds
console.time("readFileSync");
for (let i = 0; i < 1000000; i++) {
  fs.readFileSync(path)
    .toString()
    .split("\n")
    .filter((_, idx) => idx % 2 === 0)
    .forEach((_) => (temp = "LINE"));
}

// Read every other line
fs.readFileSync(path)
  .toString()
  .split("\n")
  .filter((_, idx) => idx % 2 === 0)
  .forEach((line) => console.log(line));

console.timeEnd("readFileSync");
