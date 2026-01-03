# Writing Tests

## Integration Testing & Contract Adherence

Beyond writing tests for the main purpose of an observable or operator, you
must also include tests for other standard behavior, such as making sure
teardowns were forwarded and executed on unsubscribe, and that resources are
disposed even when downstream unsubscribed after an action.

The more complex an observable/operator is, the easier it is to accidentally
break an expected standard behavior. That is why it's important to test for
**all** of these requirements, even if the observable/operator itself does not
need to consider them in its implementation due to its simplicity.

> If you develop an observable/operator outside of this repository, you do not
> need to follow the standard test structure this repository is using, but it is
> recommended. To ensure your observable/operator behaves as expected, it's
> enough to follow and test for the [Runtime Contracts](./contracts.md).

## Test Organization

> See also: [Rust Book / Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)

Contract testing is integration testing, and it should be done from the same
"outside" perspective as the users would use your observable/operator.

- Integration tests should be implemented in the `tests` folder of your crate.
- Unit tests can be put anywhere; having them in the same file as the thing
   tested is preferred, as it gives you access to its private internals for
   assertions.

## Code Coverage & Dead Code Elimination

Always start with integration testing first! This gives you an opportunity to
see if your implementation has any dead code: code that doesn't even run while
still operating correctly.

Since at this point you've only tested your observable/operator from the
outside, now you can evaluate what to do with code that was not covered by
your tests depending on whether or not it's even possible to reach it:

- If it's possible and is related to your logic, you've missed testing a
  feature!
- If it's possible and is not related to your logic but to standard behavior,
  and the contract tests didn't cover it, then you found an edge case and a
  new contract should be added!
- If it's not possible to reach it, deleting it depends on whether
  that piece of code has any purpose when used somewhere other than your
  operator/observable. If it's a subscriber exported by your crate, someone
  else may write another operator with it that does make that piece of code
  reachable. Depending on this, you may either write a unit test for it or
  delete the useless code.

### What `100%` Test Coverage Means

> In general

It's very important to recognize that `100%` test coverage **does not** mean
your project is completely bug free! It means that it does not have any dead
code, and that every feature is at least partially tested!

It's still very useful, as it gives you confidence that new changes will not
break existing features, and your users can be confident that at least one
test exists for any feature they may end up using.

Code coverage should be thought of as an outward-facing metric for users. A
confidence score of sorts. Therefore it's not required to include code in the
coverage that will never reach a user. For example, private crates that are
usually dev tools are irrelevant for the user and can safely be excluded from
code coverage.

Requiring it at a CI/CD level also gives you some benefits, not just in teams
but for solo projects as well:

- It forces you to write tests for every feature, eliminating the possibility
  of forgetting to test something! (Again, it does not mean you have covered
  all edge cases, only that it's at least tested!)
