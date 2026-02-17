# Planning

- Reproduce current behavior and run baseline tests.
- Identify where find-mode input is mutated and where rendered state is read.
- Apply the smallest fix that keeps existing mode structure intact.
- Add one focused regression test for find-mode character input persistence.
- Document Rc/RefCell usage and alternatives without broad refactors in this change.
