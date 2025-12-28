#!/usr/bin/env bun

import { connectable, finalize, ReplaySubject, Subject } from "rxjs";

const source = new Subject<number>();

const connectableObservable = connectable(
  source.pipe(finalize(() => console.log("source finalize"))),
  {
    connector: () => {
      console.log("create connector");
      return new ReplaySubject<number>(1);
    },
    resetOnDisconnect: false,
  }
);

const subscription1 = connectableObservable
  .pipe(finalize(() => console.log("connection finalize 0")))
  .subscribe({
    next: (next) => console.log("connectable observable 0 - next", next),
    complete: () => console.log("connectable observable 0 - complete"),
  });

const connection = connectableObservable.connect();

source.next(1);
source.complete();

const subscription2 = connectableObservable
  .pipe(finalize(() => console.log("connection finalize 1")))
  .subscribe({
    next: (next) => console.log("connectable observable 1 - next", next),
    complete: () => console.log("connectable observable 1 - complete"),
  });

console.log("end");
