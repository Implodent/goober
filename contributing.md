# Contribution guide

- Fork the project, work on your contribution.
- Run `cargo build` to verify that your changes compile.
- Submit a pull request, and wait for the maintainers to review it.
- Congratulations! You have now contributed to this project.

## Notes on codebase, questionable choices and behavior, etc.

- Any panic in the render/measure code should only happen if failing to do some operation is a bug.
  Especially in terminal code, where you *can* return an error, but for example not being able to convert f32's (Taffy x,y etc.) to u16's (crossterm row,column) is a bug, as it only happens in two cases: the value is out of bounds for a u16 (more than 16k pixels, woooah), or it is negative, which, again, should never happen.
  So we panic in this case.
