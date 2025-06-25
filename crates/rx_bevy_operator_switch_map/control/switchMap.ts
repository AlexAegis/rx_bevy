#!/usr/bin/env bun

import { map, of, switchMap } from "rxjs";

of(1, 2, 3, 4, 5)
  .pipe(
    switchMap((next) => {
      console.log("fromof", next);
      return of(1, 2, 3).pipe(
        map((i) => {
          console.log("inner map", i);
          return `hello ${i}`;
        })
      );
    })
  )
  .subscribe((i) => console.log("sub", i));
