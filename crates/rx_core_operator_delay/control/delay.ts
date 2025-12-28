#!/usr/bin/env bun
import { delay, Subject } from "rxjs";

const source = new Subject();

const sub = source.pipe(delay(1000)).subscribe({
  next: (n) => console.log("next", n),
  error: (n) => console.log("error", n),
  complete: () => console.log("complete"),
});

source.next(1);
setTimeout(() => {
  source.complete();
  source.next(2);
}, 999);
