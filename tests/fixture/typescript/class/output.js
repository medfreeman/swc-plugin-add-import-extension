import { test } from "./entry/index.mjs";

export class TestClass {
  static postRaw(req, res) {
    const chartData = req.body?.chart;
    if (!req.body || !chartData || !chartData?.data.length) {
      //
    }
  }
}
