var last_practicing_file: string | null = null

function openLastFile() {

}

function openFileSelector() {

}

setup().then((last_file) => {

    //We are skipping null checks to speed up development time
    let last_file_path_text = document.getElementById("last-file-path")!;
    let last_file_path_button = document.getElementById("last-file-button") as HTMLButtonElement;

    if (last_file != null) {
        last_file_path_text.innerText = last_file;
        last_file_path_button.disabled = false;
    } else {
        last_file_path_text.innerText = "No file was opened before";
        last_file_path_button.disabled = true;

    }
    last_practicing_file = last_file
}, (err) => {
    showFatalError(err);
})
