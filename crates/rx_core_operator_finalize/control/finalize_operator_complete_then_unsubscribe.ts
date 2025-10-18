#!/usr/bin/env bun

import { finalize, Subject } from "rxjs";

const subject = new Subject();

const subscription = subject
  .pipe(finalize(() => console.log("finalize!")))
  .subscribe({
    complete: () => console.log("complete"),
  });

subject.complete();
subscription.unsubscribe();

/*
Output:
complete
finalize
*/
