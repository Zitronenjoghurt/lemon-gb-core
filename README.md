# lemon-gb-core
A try at rewriting my current GB emulator implementation to be more accurate and more maintainable from the start.

## New Ideas
- CPU as the main driver of the system
  - The instruction execute phase will be able to tick cycles
  - When cycles are ticked, all other components will be ticked
- Cycle Accuracy
  - 