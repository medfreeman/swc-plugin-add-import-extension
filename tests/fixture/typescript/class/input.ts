import { test } from "./entry";

export class TestClass {
  public static postRaw(req: Request<RawRenderRequest>, res: Response) {
    const chartData = req.body?.chart;
    if (!req.body || !chartData || !chartData?.data.length) {
      //
    }
  }
}
