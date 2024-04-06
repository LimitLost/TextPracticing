/**
 * 0 - not done auto select
 * 1 - done auto select
 */
var only_one_selection: number | null = null

function update_possible_selections(not_done_possible: boolean, done_possible: boolean) {
    not_done_button.disabled = !not_done_possible;
    done_button.disabled = !done_possible;
    //At least one of these bools must be true, it's checked on the tauri side
    if (not_done_possible != done_possible) {
        only_one_selection = not_done_possible ? 0 : 1;
    } else {
        only_one_selection = null
    }
}

function reset_learning_panel(subject: string) {
    for (const el of document.getElementsByClassName("subject-title")) {
        (<HTMLElement>el).innerText = subject;
    }

    learning_phase_fields.innerHTML = ""
    testing_table.innerHTML = ""

}

function create_learning_panel_field(title: string, data: string) {
    //Table rows for testing panel
    let name_tr = document.createElement('tr');
    let data_tr = document.createElement('tr');

    name_tr.innerHTML = `
    <td class="field-title"></td>
    <td>(Original)</td>`;

    data_tr.innerHTML = `
    <td>
      <div class="field-data" contenteditable="true"></div>
    </td>
    <td>
      <div class="field-data original-text" contenteditable="true"></div>
    </td>`;

    (<HTMLElement>name_tr.getElementsByClassName("field-title")[0]).innerText = title;

    (<HTMLElement>data_tr.getElementsByClassName("original-text")[0]).innerText = data;

    //Field for learning panel
    let learning_div = document.createElement('div');

    learning_div.className = "field column"

    learning_div.innerHTML = `
        <div class="field-title"></div>
        <div class="field-data" contenteditable="true"></div>`;

    (<HTMLElement>learning_div.getElementsByClassName("field-title")[0]).innerText = title;
    (<HTMLElement>learning_div.getElementsByClassName("field-data")[0]).innerText = data;

    //Add created elements into table and field list
    learning_phase_fields.appendChild(learning_div);
    testing_table.appendChild(name_tr);
    testing_table.appendChild(data_tr);
}