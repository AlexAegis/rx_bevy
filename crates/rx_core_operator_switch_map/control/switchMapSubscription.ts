#!/usr/bin/env bun

import { finalize, map, Subject, switchMap } from "rxjs";

const source = new Subject<number>();

const inner = new Subject<number>();

const subscription = source
  .pipe(
    finalize(() => console.log("outer before switchmap finalize")),
    switchMap((next) => {
      console.log("source next", next);
      return inner.pipe(
        map((i) => {
          console.log("inner map", next, i);
          return `hello ${next} ${i}`;
        }),
        finalize(() => console.log("inner finalize"))
      );
    }),
    finalize(() => console.log("outer after switchmap finalize"))
  )
  .subscribe((i) => console.log("sub", i));

console.log("--- step 1");
source.next(1);
console.log("--- step 2");

inner.next(1);
console.log("--- step 3");
source.next(2);
console.log("--- step 4");

inner.next(2);
console.log("--- step 5");

source.complete(); // only the first finalize ran!!
console.log("--- step 6");

source.unsubscribe(); // no finalize ran!!
