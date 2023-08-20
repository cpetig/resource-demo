import { MyObject } from 'test:example/my-interface';

let o = new MyObject(42)
o.set(17);
o.set(o.get() * 2);
console.log('current value', o.get());
