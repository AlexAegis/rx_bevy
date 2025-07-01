#!/usr/bin/env bun
import { combineLatest, from } from "rxjs";

/**
 * The combineLatest observer combines the latest values from multiple observables
 * Notice that in the output, 1, and 2 is not present, that's because
 * the first observable emits all of its values immediately upon subscription,
 * before the second one could even start listening.
 */

const observable1 = from([1, 2, 3]);
const observable2 = from([4, 5, 6]);

combineLatest([observable1, observable2]).subscribe({
  next: (next) => console.log(next),
});
