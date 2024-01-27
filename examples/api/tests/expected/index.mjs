import { TestClass, helloWorld } from "./utils/index.mjs";
export function logHelloWorld() {
    new TestClass();
    console.log(helloWorld());
    const bla = {
        foo: 'bar'
    };
}
