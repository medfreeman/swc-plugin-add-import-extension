import { stdin } from "node:process";
import { createInterface } from "node:readline";

export async function readPipe() {
  const rl = createInterface({
    input: stdin,
  });

  let lines = [];

  for await (const line of rl) {
    lines.push(line);
  }

  return lines;
}
