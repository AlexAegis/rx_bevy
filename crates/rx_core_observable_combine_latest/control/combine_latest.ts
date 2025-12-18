#!/usr/bin/env bun
import { combineLatest, Subject } from "rxjs";

const observable1 = new Subject();
const observable2 = new Subject();

combineLatest([observable1, observable2]).subscribe({
  next: (next) => console.log(next),
  complete: () => console.log("complete"),
});

observable1.complete();
observable2.unsubscribe();
