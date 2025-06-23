#!/usr/bin/env bun

import { defer, of } from "rxjs";

let source = of(1);

let deferred = defer(() => source);

deferred.subscribe({
  next: (next) => console.log(next),
});

deferred.subscribe({
  next: (next) => console.log(next),
});
