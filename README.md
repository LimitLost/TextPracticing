# Text Practicing

This project is meant to help me practice for the final polish exam (Matura ustna). It's currently pretty personalized so you will probably to need change the code for it to work on your case.

Based on [Tauri](https://tauri.app/)

## How It Works

- Loads all the subject data from the selected file
- Selects a random subject from the loaded list that wasn't shown before (or was shown before, depending on the button selected, if possible)

### Learning Phase

- Entered right after subject question is selected

- Shows the Question Data (Currently: Wstęp, Teza, Odwołanie, Kontekst, Zakończenie)

- You can now start the test or wait additional selected time before the test start (useful for testing long term memory)

### Testing Phase

- You will be shown the empty text boxes to fill
- You can always come back to the learning phase
- 10 minute countdown will be shown
- After clicking the `Done` button you will be moved into the `Check Results Phase`

### Check Results Phase

- You will be shown the original learning text for comparison
- You can now start learning current subject again or select a new random one, that was or wasn't shown before

## File Parsing

[Regular Expression File](/src-tauri/src/regex) with comments

[Example file](/src-tauri/src/example-file.txt) working for the current Regex

## Compilation

Requirements:

- [Rust Language](https://www.rust-lang.org/learn/get-started)
- [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)

Only needed for the first compilation:

```bat
git clone https://github.com/LimitLost/TextPracticing.git
cd TextPracticing
npm install
```

Compilation:

```bat
npm run tauri dev
```

## License

[MIT](/LICENSE)

Logo: CC-BY-NC-ND

- Original Tauri Logo Designs by Daniel Thompson-Yvetot and Guillaume Chau
