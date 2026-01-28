# Feature Backlog TODO:

Document describing the planned features and their progress status.

## Feature Summary Table

Priority levels: High, Medium, Low
Status levels: Not Planned, Planning-Candidate, Planning, Planned, In Progress, On-Hold, Completed

| Feature                         | Priority | Status             |
|---------------------------------|----------|--------------------|
| File import                     | High     | On-Hold            |
| Go-to line functionality        | High     | Planning           |
| Log viewer's file search/find   | High     | Planning           |
| Partial log file loading        | High     | Planning           |
| Application color scheme        | High     | Planning           |
| Application color theme support | Medium   | Planning-Candidate |
| Multiple files support          | Low      | Planning-Candidate |

## Feature Plan

- [ ] Refer to detailed plans in `docs/feature-plans/README.md`
- [ ] File import - Priority: High
    - [x] import from command line argument
    - shortcut to open a dialog for importing a file
    - [x] concurrently get the file and load it to the tui log viewer
    - [ ] dialog to select/input the file to import
    - [ ] dialog should inform the user if the input file is invalid, or path is invalid, idk.
- [ ] Go-to line functionality - Priority: High
    - [ ] implement go-to line input-dialog/input-box(can be over of in the side/bottom)
    - [ ] implement the logic to jump to the specified line in the log viewer
- [ ] Log viewer's file search/find - Priority: High
    - TODO: To define.. since a lot of topics need to be debated like:
    - should it be a dialog or another box at the bottom(or other place)?
    - should it be regex based?
    - should it support case sensitivity?
    - should it support whole word search?
    - should it support search history?
    - should it highlight all matches or just the current match? (or highlight in a different way each)
    - should it support next/previous match navigation?
    - should it support search within a selected range?
    - should it support search filters (e.g. log level, date range)?
    - should it support live search (i.e. update results as user types)?
- [ ] Partial log file loading - should now have the whole file in memory - Priority: High
    - have a way to harmonize the way the search/find functionality works with partial file loading
- [ ] Application color scheme - Priority: High
    - Have a visually good first color scheme
- [ ] Application color theme support - Priority: Medium
    - support at least one theme (e.g. catppuccin)
- [ ] Multiple files support - Priority: Low
    - open multiple files and switch between them
