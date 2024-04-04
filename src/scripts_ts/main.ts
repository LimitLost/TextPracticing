var last_practicing_file: string | null = null

function openLastFile() {

}

function openFileSelector() {

}

function doneSelect() {
    //TODO Reset Visibility of testing and learning panels
    hide(testing_phase_panel)
    show(learning_phase_panel)
}

function notDoneSelect() {
    //TODO Reset Visibility of testing and learning panels
    hide(testing_phase_panel)
    show(learning_phase_panel)
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
