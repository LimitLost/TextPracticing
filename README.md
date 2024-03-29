# Text Practicing

This project is meant to help me practice for the final polish exam (Matura ustna). It's currently pretty personalized so you will probably to need change the code for it to work on your case.

Based on [Tauri](https://tauri.app/)

## How It Works

- Loads all the subject data from selected file
- Selects a random subject from the loaded list that wasn't shown before (or was shown before, depending on the button selected)

### Learning Phase

- Entered right after subject question is selected

- Shows the Question Data (Currently: Wstęp, Teza, Odwołanie, Kontekst, Zakończenie)

- You can now start the test or wait additional selected time before the test start (usefull for testing long term memory)

### Testing Phase

- You will be shown the empty text boxes to fill
- You can always come back to the learning phase
- After clicking the `Done` button you will be moved into the `Check Results Phase`

### Check Results Phase

- You will be shown the original learning text for comparison
- You can now start learning current subject again or select a new random one, that was or wasn't shown before

## License

MIT
