#!/usr/bin/env bun

import { delay, finalize, Subject, tap } from "rxjs";

const subject = new Subject<number>();

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

subject.next(1);
subject.complete();
subject.unsubscribe();
// subject.next(13232);

// > Output:
// error
// unsubscribe
