/**
 *
 * @param {HTMLElement} el
 */
function hide(el) {
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
    }
    el.classList.remove("fully-hidden", "hidden");
}

function startGlobalLoading() {
    let panel = document.getElementById("global-loading")

    show(panel);
}

function stopGlobalLoading() {
    let panel = document.getElementById("global-loading")

    hide(panel);

}

function showFatalError(err) {
    console.error(err);
    let panel = document.getElementById("err-panel-base");
    let text = document.getElementById("err-text");
    let hideButton = document.getElementById("err-button");
    text.innerText = err;
    hideButton.style.visibility
    show(panel);
}
/**
 * 
 * @param {*} err 
 * @param {()=>any} hide_action 
 */
function showError(err, hide_action = undefined) {
    console.error(err);
    let panel = document.getElementById("err-panel-base");
    let text = document.getElementById("err-text");
    let hideButton = document.getElementById("err-button");
    if (typeof hide_action == 'function') {
        hideButton.innerText = "Try Again"
        hideButton.onclick = () => {
            hideErrPanel();
            hide_action();
        }
    } else {
        hideButton.innerText = "OK"
        hideButton.onclick = () => {
            hide_action();
        }
    }

    text.innerText = err;
    show(panel);
}
function hideErrPanel() {
    let panel = document.getElementById("err-panel-base");
    hide(panel);
}