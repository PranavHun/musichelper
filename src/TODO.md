Data
Json + an option to import in a database.
Option to View raw data in a report.

Logic
Separate Fretboard logic like highlight logic.
Main Program just passes the args and passes various arguments individually.
tuning name, key, etc.

Display
Display should get only Fret Notes HashMap<TuningName, Vec<Vec<String>>>, Highlight Notes HashMap<highlightName, Vec<String>> (Set)
In text display each Highlight Notes Vec will print a new fret board.
