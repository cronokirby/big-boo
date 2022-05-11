# Design

This section is about the design of the Boo BC bytecode. Why are
certain features included, why are others exluded? This is the kind of question
we hope to answer in this section.

Boo BC tries to satisfy two goals at the same time. On one hand, we need
to provide a complete set of instructions which should be a reasonable target 
for higher level languages, or other targets like WASM. On the other,
we need to provide operations which closely map to the execution environment
we need in the MPC-in-the-head style of proofs we use.

We're somewhat free in terms of the execution environment. One thing
that's made very difficult though is any kind of dynamic control flow.
Branching is tedious, and so is dynamic memory accesses. Static accesses,
on the other hand, are fine.

Another quirk of the execution environment, because of the particular
implementation of proofs we have, is that it's easy to either do
bit operations on a value, or arithmetic operations on a value, but not
both in succession. Doing that involves an expensive conversion
operation. So, it's better to convert once, and then keep doing one
kind of operation on that value.

Because of these constraints, we decided to go with a stack-based bytecode,
with multiple execution stacks.
