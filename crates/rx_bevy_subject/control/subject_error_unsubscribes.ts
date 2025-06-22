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

// > Output:
// error
// unsubscribe
