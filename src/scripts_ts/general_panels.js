/**
 *
 * @param {HTMLElement} el
 */
function hide(el) {
    if (el.classList.contains("hidden")) {
        //Do nothing when element is already hidden
        return;
    }
    el.classList.add("hidden");
    if (el.classList.contains("can-hide")) {
        let timeoutId = setTimeout(() => {
            el.removeAttribute("hiding-id");
            el.classList.add("fully-hidden");
        }, 100);
        el.setAttribute("hiding-id", timeoutId);
    }
    else {
        el.classList.add("fully-hidden");
    }
}
/**
*
* @param {HTMLElement} el
*/
function show(el) {
    if (el.hasAttribute("hiding-id")) {
        clearTimeout(Number(el.getAttribute("hiding-id")));
        el.removeAttribute("hiding-id");
    }
    el.classList.remove("fully-hidden", "hidden");
}

function startGlobalLoading() {

    show(global_loading);
}

function stopGlobalLoading() {

    hide(global_loading);

}

function showFatalError(err) {
    console.error(err);

    err_text.innerText = err;
    err_button.style.visibility = "hidden";
    show(err_panel_base);
}
/**
 * 
 * @param {*} err 
 * @param {()=>any} hide_action 
 */
function showError(err, hide_action = undefined) {
    console.error(err);
    err_button.style.visibility = null;
    if (typeof hide_action == 'function') {
        err_button.innerText = "Try Again"
        err_button.onclick = () => {
            hideErrPanel();
            hide_action();
        }
    } else {
        err_button.innerText = "OK"
        err_button.onclick = () => {
            hide_action();
        }
    }

    err_text.innerText = err;
    show(err_panel_base);
}
function hideErrPanel() {
    hide(err_panel_base);
}