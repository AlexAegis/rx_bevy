#!/usr/bin/env bun

import { finalize, Subject, take } from "rxjs";

const subject = new Subject();

const subscription = subject
  .pipe(
    finalize(() => console.log("finalize!")),
    take(3)
  )
  .subscribe({
    complete: () => console.log("complete"),
  });

subscription.add(() => {
  console.log("teardown");
});

subject.next(1);
subject.next(2);
subject.next(3);
subject.next(4);
subscription.unsubscribe();

/*
Output:
complete
finalize
*/
