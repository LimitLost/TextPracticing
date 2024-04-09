const { invoke } = window.__TAURI__.tauri;

window.invoke = invoke;

/**
 * 
 * @param {string|null} defaultPath 
 * @param {string} title 
 * @returns {Promise<null | string | string[]>}
 */
function windowFileOpen(defaultPath, title) {
    window.tauri_file_dialog_open({
        defaultPath: defaultPath,
        title: title
    })
}

window.windowFileOpen = windowFileOpen;

