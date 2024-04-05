var last_practicing_file: string | null = null

function openLastFile() {

}

function openFileSelector() {
    //TODO https://tauri.app/v1/api/js/dialog/#opendialogoptions
    //TODO https://beta.tauri.app/features/dialog/

    let open_fn = () => {

        let selected = await windowFileOpen(last_practicing_file, "Open file with practice data")
        if (selected != null) {

        }

        windowFileOpen(last_practicing_file, "Open file with practice data").then((value) => {
            open_file(<string>value)
        }, (err) => {
            showError(err, open_fn)
        })
    }

    open_fn();


}

function doneSelect() {
    //Reset Visibility of testing and learning panels
    hide(testing_phase_panel)
    show(learning_phase_panel)

    //TODO
}

function notDoneSelect() {
    //Reset Visibility of testing and learning panels
    hide(testing_phase_panel)
    show(learning_phase_panel)

    //TODO
}

function startTest() {

}
function waitStartTest() {

}
function cancelPause() {

}
function startPause() {

}
function backToLearning() {

}
function completeLearning() {

}
function learnAgain() {

}

setup().then((last_file) => {
    if (last_file != null) {
        last_file_path.innerText = last_file;
        last_file_button.disabled = false;
    } else {
        last_file_path.innerText = "No file was opened before";
        last_file_button.disabled = true;
    }
    last_practicing_file = last_file
}, (err) => {
    showFatalError(err);
})
