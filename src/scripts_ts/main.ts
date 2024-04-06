var last_practicing_file: string | null = null

function openLastFile() {

}

function openFileSelector() {

    let action = async () => {

        let selected = null;

        while (selected == null) {
            selected = <string | null>await windowFileOpen(last_practicing_file, "Open file with practice data")
        if (selected != null) {
                await open_file(selected)
                if (only_one_selection != null) {
                    switch (only_one_selection) {
                        case 0:
                            notDoneSelect();
                            break;
                        case 1:
                            doneSelect();
                            break;
                        default:
                            console.log(`only_one_selection: ${only_one_selection}`);
                            showError("Unknown default subject type detected!");
                            openTypeSelector();
                    }
                } else {
                    openTypeSelector();
                }
            }
        }

        }


    let run = () => {
        startGlobalLoading();
        action().then(() => {
            stopGlobalLoading();
        }, (err) => {
            stopGlobalLoading();
            showError(err, run)
        })
    }

    run();
}

function openTypeSelector() {
    show(select_subject_type_panel)
}

function subjectSelected(done: boolean) {
    //Reset Visibility of testing and learning panels
    hide(testing_phase_panel)
    show(learning_phase_panel)

    hide(select_subject_type_panel)


    let action = () => {
        startGlobalLoading();
        open_random_subject(done).then(() => {
            stopGlobalLoading();
            show(learning_phase_panel);
        }, (err) => {
            stopGlobalLoading();
            showError(err, action)
        })

}

    action();
}

function doneSelect() {
    subjectSelected(true);
}

function notDoneSelect() {
    subjectSelected(false);
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
