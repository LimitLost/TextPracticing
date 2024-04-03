/**
 * 0 - not done auto select
 * 1 - done auto select
 */
var only_one_selection: number | null = null

function update_possible_selections(not_done_possible: boolean, done_possible: boolean) {
    let not_done_button = document.getElementById("not-done-button") as HTMLButtonElement;
    let done_button = document.getElementById("done-button") as HTMLButtonElement;

    not_done_button.disabled = !not_done_possible;
    done_button.disabled = !done_possible;
    //At least one of these bools must be true, it's checked on the tauri side
    if (not_done_possible != done_possible) {
        only_one_selection = not_done_possible ? 0 : 1;
    }
}

function reset_learning_panel(subject: string) {
    //TODO Update learning-phase-fields and same thing in testing panel
}

function create_learning_panel_field(title: string, data: string) {

}