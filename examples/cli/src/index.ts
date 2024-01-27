import { TestClass, helloWorld } from "~utils";

export function logHelloWorld() {
  new TestClass();
  console.log(helloWorld());
  const bla = { foo: 'bar' };
}
