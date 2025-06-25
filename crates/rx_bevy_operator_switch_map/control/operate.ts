#!/usr/bin/env bun

import { Subject, Subscriber } from "rxjs";

const destination = new Subject();
destination.subscribe(console.log);

const subscriber = new Subscriber(destination);

subscriber.next(1);

console.log(
  "subscriber closed",
  subscriber.closed,
  "subject closed",
  destination.closed
);
subscriber.unsubscribe();

console.log(
  "subscriber closed",
  subscriber,
  "subject closed",
  destination.closed
);
