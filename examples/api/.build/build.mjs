#!/usr/bin/env node
import { writeFile } from "node:fs/promises";
import { dirname, basename, extname, join, relative } from "node:path";
import { cwd } from "node:process";

import { transformFile } from "@swc/core";
import { execute } from "@yarnpkg/shell";
import fsExists from "fs.promises.exists";

import { readPipe } from "./readPipe.mjs";

import swcConfig from "../.swcrc.json" assert { type: "json" };

const processedSwcConfig = {
  ...swcConfig,
  jsc: {
    ...swcConfig.jsc,
    baseUrl: join(cwd(), swcConfig.jsc?.baseUrl ?? "."),
  }
};

const outputFolder = "dist";

function transpileFileFactory({ swcConfig: _swcConfig }) {
  return async function transpileFile(inputFilePath) {
    const { code, map } = await transformFile(inputFilePath, _swcConfig);
    return { code, map };
  };
}

const entryPoints = await readPipe();

const transpileFile = transpileFileFactory({ swcConfig: processedSwcConfig });

await Promise.all(entryPoints.map(async (entryPoint) => {
  const { code } = await transpileFile(entryPoint);

  const relativeFileSourcePath = relative("src", entryPoint);
  const relativeFileOutputPath = `${join(outputFolder, dirname(relativeFileSourcePath), basename(relativeFileSourcePath, extname(relativeFileSourcePath)))}.mjs`;
  const relativeFileOutputFolder = dirname(relativeFileOutputPath);

  if (!(await fsExists(relativeFileOutputFolder))) {
    process.exitCode = await execute(`mkdir -p`, [relativeFileOutputFolder]);
  }

  await writeFile(relativeFileOutputPath, code, { encoding: "utf8" });
}));
