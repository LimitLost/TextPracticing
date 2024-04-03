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
    for (const el of document.getElementsByClassName("subject-title")) {
        (<HTMLElement>el).innerText = subject;
    }

    //TODO Update learning-phase-fields and same thing in testing panel

    document.getElementById("learning-phase-fields")!.innerHTML = ""
    document.getElementById("testing-table")!.innerHTML = ""

}

function create_learning_panel_field(title: string, data: string) {
    let name_tr = document.createElement('tr');
    let data_tr = document.createElement('tr');
    let learning_div = document.createElement('div');

    //TODO create internal html for above variables

    let learning_phase_fields = document.getElementById("learning-phase-fields")!
    let testing_table = document.getElementById("testing-table")!

    learning_phase_fields.appendChild(learning_div);
    testing_table.appendChild(name_tr);
    testing_table.appendChild(data_tr);
}