var last_practicing_file: string | null = null

var test_timer_interval_id: number | null = null
var test_timer_start_millis: number | null = null

var pause_interval_id: number | null = null;
var pause_start_millis: number | null = null;
var pause_end_millis: number | null = null;

function nowUtcMillis() {
    let date = new Date();
    //Converted From Minutes To Milliseconds
    let timezoneDiff = date.getTimezoneOffset() * 60 * 1000
    //If you're time is later than UTC then timezoneDiff is negative
    return date.getTime() + timezoneDiff
}

async function openFile(file_path: string) {
    await open_file(file_path)
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

function openLastFile() {
    let action = async () => {
        if (last_practicing_file != null) {
            openFile(last_practicing_file)
        } else {
            showError("There is no last file open!")
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

var windowFileOpen: (defaultPath: string | null, title: string) => Promise<null | string | string[]>;

function openFileSelector() {

    let action = async () => {

        let selected = null;

        selected = await windowFileOpen(last_practicing_file, "Open file with practice data")
        console.log(selected)
        if (selected != null) {
            await openFile(<string>selected)
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


function testTimerUpdate() {
    let now = nowUtcMillis();
    let diff = now - test_timer_start_millis!

    let current_timer_millis = 600_000 - diff

    let millis = current_timer_millis % 1000;
    let seconds = (current_timer_millis - millis) / 1000;
    let shown_seconds = seconds % 60;
    if (seconds < 0) {
        shown_seconds = (-seconds) % 60;
    }
    let minutes = (seconds - (seconds % 60)) / 60;


    testing_countdown_minute.innerText = minutes.toString();
    testing_countdown_second.innerText = shown_seconds.toString().padStart(2, "0");
}

function startTest() {
    hide(learning_phase_panel);
    show(testing_phase_panel);
    //Reset And Start 10 Minute Timer Countdown
    testing_countdown_minute.innerText = "10";
    testing_countdown_second.innerText = "00";

    test_timer_start_millis = nowUtcMillis();
    //Start Check every 100ms, if it wasn't activated before
    if (test_timer_interval_id == null) {
        test_timer_interval_id = setInterval(() => {
            testTimerUpdate();
        }, 100)
    }

    //Reset Test Buttons shown
    testing_panel_buttons.style.display = "";
    //Hide Results
    check_results_panel_buttons.style.display = "none";
    testing_table.classList.add("second-hidden");

}
function waitStartTest() {
    hide(learning_phase_panel);
    show(pause_time_panel);
    //Reset Pause Time Selector
    pause_editor_row.style.display = ""
    pause_show_row.style.display = "none"

}
function cancelPause() {
    pause_editor_row.style.display = ""
    pause_show_row.style.display = "none"
    cancel_pause_button.onclick = exitPauseMenu;
    start_pause_button.disabled = false;
    if (pause_interval_id != null) {
        clearInterval(pause_interval_id);
        pause_interval_id = null
    }

}
function exitPauseMenu() {
    show(learning_phase_panel);
    hide(pause_time_panel);
}

function pauseTimerUpdate() {
    let now = nowUtcMillis();

    if (now > pause_end_millis!) {
        //Pause is complete
        clearInterval(pause_interval_id!);
        pause_interval_id = null;
        hide(pause_time_panel);
        startTest();
        return;
    }

    let current_timer_millis = pause_end_millis! - now

    let millis = current_timer_millis % 1000;
    let seconds = (current_timer_millis - millis) / 1000;
    let shown_seconds = seconds % 60;
    if (seconds < 0) {
        shown_seconds = (-seconds) % 60;
    }
    let minutes = (seconds - (seconds % 60)) / 60;


    pause_minute.innerText = minutes.toString();
    pause_second.innerText = shown_seconds.toString().padStart(2, "0");
}

function startPause() {
    //Hide Time Selector Inputs
    pause_editor_row.style.display = "none"
    //Show And Start Pause Countdown
    pause_show_row.style.display = ""
    pause_start_millis = nowUtcMillis();
    let pause_len_millis = (pause_editor_minute.valueAsNumber * 60_000) + (pause_editor_second.valueAsNumber * 1_000);

    pause_end_millis = pause_start_millis + pause_len_millis;
    pause_interval_id = setInterval(() => {
        pauseTimerUpdate();
    }, 100)
    //Disable start Pause Button
    start_pause_button.disabled = true;
    //Switch Cancel pause button functionality (from exit to stop pause)
    cancel_pause_button.onclick = cancelPause;

}

function backToLearning() {
    hide(testing_phase_panel);
    show(learning_phase_panel);

}
function completeLearning() {
    //Hide Testing Buttons
    testing_panel_buttons.style.display = "none";
    //Show Results
    check_results_panel_buttons.style.display = "";
    testing_table.classList.remove("second-hidden");
    //Pause Countdown Timer
    clearInterval(test_timer_interval_id!);
    test_timer_interval_id = null;
}
function learnAgain() {
    backToLearning();
}
startGlobalLoading();
setup().then((last_file) => {
    if (last_file != null) {
        last_file_path.innerText = last_file;
        last_file_button.disabled = false;
    } else {
        last_file_path.innerText = "No file was opened before";
        last_file_button.disabled = true;
    }
    last_practicing_file = last_file
    stopGlobalLoading();
}, (err) => {
    stopGlobalLoading();
    showFatalError(err);
})
