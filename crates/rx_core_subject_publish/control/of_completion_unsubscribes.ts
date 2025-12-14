#!/usr/bin/env bun

import { delay, finalize, of, tap } from "rxjs";

const subscription = of(1, 2)
  .pipe(
    tap({
      complete: () => {
        console.log("complete tap 1");
      },
    }),
    delay(100),
    finalize(() => console.log("finalize 1")),
    tap({
      complete: () => {
        console.log("complete tap 2");
      },
    }),
    finalize(() => console.log("finalize 2"))
  )
  .subscribe({
    complete: () => {
      console.log("complete observer");
    },
  });

console.log("sub");

subscription.unsubscribe();
console.log("unsub");

/*
complete tap 1
complete tap 2
complete observer
finalize 1
finalize 2
*/
