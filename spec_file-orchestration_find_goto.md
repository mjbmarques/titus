# Initial Spec for: File orchestration, find and goto

This spec is composed of 3 fronts:
- File orchestration
- file 'find' functionality
- file 'goto' functionality

## File Orchestration
The way files are loaded should have huge file size, hundreds of files and high performance into consideration.
By this I mean:
- huge file size: files up to 1GB should not be loaded fully; actually, no file should be loaded fully without consideration.
- hundreds of files: selecting multiple files to work as a "single log entity" should be supported.
- high performance: for the 2 mentioned points, the application should remain fast (basically instant) and should use barelly any ram, should not exceed 250mb for example, not a hard cap, but with tests, we should guarantee that.

### The following "backend" mandatory functionalities should be considered:
- A page showing logs should only have loaded in the backend the currently shown log lines plus a couple more up and down so that when scrolling the user doesn't have to wait for the program to load said lines.
- ripgrep can be used, the application can launch it in a computer that has ripgrep and do the necessary searching.
- to find lines, the application obviously should already know the like breaks that exist in a file, so the line breaks should be preloaded and be one of the only things about a file that are always loaded (while the file is open in the application).
- when in multiple files for same log mode, files ordering should be based on name ordering alphanumeric.

Should be taken into account an already existing partial implementation of some of the functionalities in the "file-orchestration" branch of this repository.


## File 'find'
- when ctrl+f is pressed, should open a dialog/modal and request the user to write the needed string in regex (the value that goes to ripgrep) and then go to the nearest find, with the option to ctrl+up and ctrl+down to go up and down the matches that are found.
- the matches should be highlighted

## File 'goto'
- then ctrl+g is pressed, should open a dialog/modal and request the used to write the line to go to (1-based) and then immedeately go to said line if possible. If the line does not exist dont quit the dialog/model yet and notify the user that an impossible/incompatible operation was requested, and explain why.

## Common
- all the UI/TUI should use the ratatui framework as the rest of the project is already using.
- any new shortcuts should be updated in the footer
- the mentioned dialog/modal in 'find' and 'goto' should have the following rules:
    - pressing 'esc' closes the dialog/modal
    - should appear in the middle of the screen
    - should contain a pseudo-title with just something like "Find" and "Goto"
    - should contain a text box
    - in error cases should just display text under the textbox in red (yes, red for now) explaining the error
    - dialog/modal shortcuts, while the modal/dialog is open, should update the footer shortcuts with the possible shortcuts; for example: if in the normal logview we will have for example the ctrl+f and ctrl+g, when the ctrl+f is pressed and opens a dialog/modal, the ctrl+f and ctrl+g and some others should disapear.
- for any visual affecting implementation, first create a number of pocs/templates(something demoable/presentable) for me to evaluate which approach to take.