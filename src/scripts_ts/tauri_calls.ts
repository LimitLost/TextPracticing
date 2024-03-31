/**
 * 
 * @returns Last practicing file open, if any
 */
async function setup(): Promise<String | null> {
    return await invoke("setup", {})
}

async function open_file(file_path: String) {
    return await invoke("open_file", { filePath: file_path })
}
async function open_random_subject(done: boolean) {
    return await invoke("open_random_subject", { done: done })
}
async function subject_done() {
    return await invoke("subject_done", {})
}
/**
 * 
 * @param new_wait_time - in seconds
 */
async function cache_update_last_wait_time(new_wait_time: number) {
    return await invoke("cache_update_last_wait_time", { newWaitTime: new_wait_time })
}
async function cache_get_last_wait_time(): Promise<number | null> {
    return await invoke("cache_get_last_wait_time", {})
}
async function cache_get_last_file_path(): Promise<string | null> {
    return await invoke("cache_get_last_file_path", {})

}