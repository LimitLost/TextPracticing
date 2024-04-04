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
var edit_pause_minute = document.getElementById("edit-pause-minute")!
var edit_pause_second = document.getElementById("edit-pause-second")!
var pause_minute = document.getElementById("pause-minute")!
var pause_second = document.getElementById("pause-second")!

var testing_phase_panel = document.getElementById("testing-phase-panel")!
var testing_countdown_minute = document.getElementById("testing-countdown-minute")!
var testing_countdown_second = document.getElementById("testing-countdown-second")!
var testing_table = document.getElementById("testing-table")!


var global_loading = document.getElementById("global-loading")!
var err_panel_base = document.getElementById("err-panel-base")!;
var err_text = document.getElementById("err-text")!;
var err_button = document.getElementById("err-button")! as HTMLButtonElement;