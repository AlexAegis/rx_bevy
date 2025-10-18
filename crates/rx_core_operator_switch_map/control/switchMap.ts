#!/usr/bin/env bun

import { finalize, from, map, of, switchMap } from "rxjs";

of(1, 2, 3)
  .pipe(
    finalize(() => console.log("outer before switchmap finalize")),
    switchMap((next) => {
      console.log("fromof", next);
      return from(Array.from({ length: 4 - next }, (x, i) => i + 1)).pipe(
        map((i) => {
          console.log("inner map", i);
          return `hello ${i}`;
        }),
        finalize(() => console.log("inner finalize"))
      );
    }),
    finalize(() => console.log("outer after switchmap finalize"))
  )
  .subscribe((i) => console.log("sub", i));
