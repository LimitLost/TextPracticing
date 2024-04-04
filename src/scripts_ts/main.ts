var last_practicing_file: string | null = null

function openLastFile() {

}

function openFileSelector() {

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
