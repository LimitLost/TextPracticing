const { invoke } = window.__TAURI__.tauri;

const { open } = window.__TAURI__.dialog;

window.invoke = invoke;

/**
 * 
 * @param {string|null} defaultPath 
 * @param {string} title 
 * @returns {Promise<null | string | string[]>}
 */
async function windowFileOpen(defaultPath, title) {
    await open({
        defaultPath: defaultPath,
        title: title
    })
}

window.windowFileOpen = windowFileOpen;

