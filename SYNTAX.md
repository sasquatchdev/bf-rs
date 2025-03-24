# Syntax

A spec of all implemented syntactical features.

| Version | Command | Feature | Description | Notes |
| ------- | ------- | ------- | ----------- | ----- |
| _None_ | `>` | ptr | Move ptr right |
| _None_ | `<` | ptr | Move ptr left |
| _None_ | `+` | val | Increment cell under ptr |
| _None_ | `-` | val | Decrement cell under ptr |
| _None_ | `.` | out | Output the cell under ptr as char |
| _None_ | `:` | out | Output the cell under ptr as raw | unofficial |
| _None_ | `,` | in | Input char and store to cell |
| _None_ | `;` | in | Input raw and store to cell | unofficial |
| _None_ | `[` | loop | Jump past matching `]` if cell at ptr is `0` |
| _None_ | `]` | loop | Jump back into matching `[` if cell at ptr is `0` |
