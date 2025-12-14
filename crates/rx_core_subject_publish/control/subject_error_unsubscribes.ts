#!/usr/bin/env bun

import { finalize, Subject } from "rxjs";

const subject = new Subject<number>();

subject.pipe(finalize(() => console.log("unsubscribe"))).subscribe({
  error: (error) => {
    console.log(error);
  },
  complete: () => {
    console.log("complete");
  },
});

subject.error("error");
subject.unsubscribe();
subject.subscribe((a) => console.log(a));
// subject.next(13232);

// > Output:
// error
// unsubscribe
