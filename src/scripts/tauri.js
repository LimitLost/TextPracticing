import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';

window.invoke = invoke;



/**
 * 
 * @param {string|null} defaultPath 
 * @param {string} title 
 * @returns {Promise<null | string | string[]>}
 */
async function windowFileOpen(defaultPath, title) {
    return await open({
        defaultPath: defaultPath,
        title: title
    })
}

window.windowFileOpen = windowFileOpen;

