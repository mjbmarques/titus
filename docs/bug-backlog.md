# Reported Bug Backlog TODO:

Document describing the reported bugs and their progress status.

- [x] When in last line of the file, the down key is directing to the first line in the file. We do not want that.
- [ ] (maybe feature not bug) Searching for a line number (one of the last) in a file of 16 GB is taking almost minute.
  - A possible solution would be to have multiple readers open along the "checkpoints" of a file.