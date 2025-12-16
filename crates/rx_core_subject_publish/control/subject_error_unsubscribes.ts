#!/usr/bin/env bun

import { AsyncSubject, delay, finalize, tap } from "rxjs";

const subject = new AsyncSubject<number>();

subject
  .pipe(
    tap((a) => console.log("tap", a)),
    delay(1000),
    finalize(() => console.log("unsubscribe"))
  )
  .subscribe({
    error: (error) => {
      console.log(error);
    },
    complete: () => {
      console.log("complete");
    },
  });

subject.subscribe((a) => console.log(a));

subject.complete();
subject.unsubscribe();
// subject.next(13232);

// > Output:
// error
// unsubscribe
