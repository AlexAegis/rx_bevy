#!/usr/bin/env bun
import { finalize, of, Subject, zip } from "rxjs";

/**
 * The combineLatest observer combines the latest values from multiple observables
 * Notice that in the output, 1, and 2 is not present, that's because
 * the first observable emits all of its values immediately upon subscription,
 * before the second one could even start listening.
 */

const subject1 = new Subject<number>();
const subject2 = new Subject<number>();

zip([subject1, subject2])
  .pipe(
    finalize(() => {
      console.log("finalize");
    })
  )
  .subscribe({
    next: (next) => console.log(next),
    complete: () => console.log("complete"),
  });

subject1.next(1);
subject2.next(10);
subject2.next(20);

subject1.next(2);
subject1.next(3);

// Even though the other subject does not complete, this one does, and since
// nothing is left in the queue of this observable, no matter what the other
// observable emits, the zip can no longer emit anything, so it completes.
subject1.complete();

// Even if the last emission of subject 1 was consumed after it was completed!
subject2.next(30);

of(1)
  .pipe(
    finalize(() => {
      console.log("of finalize");
    })
  )
  .subscribe({
    next: (next) => console.log("of", next),
    complete: () => console.log("of complete"),
  });
