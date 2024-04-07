//We are skipping null checks to speed up development time

var select_file_panel = document.getElementById("select-file-panel")!
var last_file_button = document.getElementById("last-file-button")! as HTMLButtonElement
var last_file_path = document.getElementById("last-file-path")!

var select_subject_type_panel = document.getElementById("select-subject-type-panel")!
var not_done_button = document.getElementById("not-done-button")! as HTMLButtonElement
var done_button = document.getElementById("done-button")! as HTMLButtonElement

var learning_phase_panel = document.getElementById("learning-phase-panel")!
var learning_phase_fields = document.getElementById("learning-phase-fields")!

var pause_time_panel = document.getElementById("pause-time-panel")!
var pause_editor_minute = document.getElementById("pause-editor-minute")! as HTMLInputElement
var pause_editor_second = document.getElementById("pause-editor-second")! as HTMLInputElement
var pause_minute = document.getElementById("pause-minute")!
var pause_second = document.getElementById("pause-second")!
var pause_editor_row = document.getElementById("pause-editor-row")!
var pause_show_row = document.getElementById("pause-show-row")!
var cancel_pause_button = document.getElementById("cancel-pause-button")! as HTMLButtonElement
var start_pause_button = document.getElementById("start-pause-button")! as HTMLButtonElement

var testing_phase_panel = document.getElementById("testing-phase-panel")!
var testing_countdown_minute = document.getElementById("testing-countdown-minute")!
var testing_countdown_second = document.getElementById("testing-countdown-second")!
var testing_table = document.getElementById("testing-table")!
var testing_panel_buttons = document.getElementById("testing-panel-buttons")!
var check_results_panel_buttons = document.getElementById("check-results-panel-buttons")!


var global_loading = document.getElementById("global-loading")!
var err_panel_base = document.getElementById("err-panel-base")!;
var err_text = document.getElementById("err-text")!;
var err_button = document.getElementById("err-button")! as HTMLButtonElement;