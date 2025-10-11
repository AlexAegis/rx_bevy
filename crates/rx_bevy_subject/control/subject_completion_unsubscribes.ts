#!/usr/bin/env bun

import { finalize, Subject, tap } from "rxjs";

const subject = new Subject<number>();

const subscription = subject
  .pipe(
    tap({
      complete: () => {
        console.log("complete tap 1");
      },
      unsubscribe: () => {
        console.log("unsubscribe tap 1");
      },
    }),
    finalize(() => console.log("finalize 1")),
    tap({
      complete: () => {
        console.log("complete tap 2");
      },
      unsubscribe: () => {
        console.log("unsubscribe tap 2");
      },
    }),
    finalize(() => console.log("finalize 2"))
  )
  .subscribe({
    complete: () => {
      console.log("complete observer");
    },
  });

subscription.unsubscribe();

// > Output:
// complete
// unsubscribe
