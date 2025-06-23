#!/usr/bin/env bun

import { connectable, finalize, Subject } from "rxjs";

const source = new Subject<number>();

const connectableObservable = connectable(
  source.pipe(finalize(() => console.log("source finalize"))),
  {
    connector: () => {
      console.log("create connector");
      return new Subject<number>();
    },
    resetOnDisconnect: true,
  }
);

source.next(1);

const subscription = connectableObservable
  .pipe(finalize(() => console.log("connection finalize 0")))
  .subscribe({
    next: (next) => console.log("connectable observable 0 - next", next),
    complete: () => console.log("connectable observable 0 - complete"),
  });

console.log("connect 0");
const connection = connectableObservable.connect();

source.next(2);

connection.unsubscribe();
const subscription2 = connectableObservable
  .pipe(finalize(() => console.log("connection finalize 1")))
  .subscribe({
    next: (next) => console.log("connectable observable 1 - next", next),
    complete: () => console.log("connectable observable 1 - complete"),
  });

source.next(3);

console.log("connect 1 (noop)");
connectableObservable.connect();

source.next(4);
