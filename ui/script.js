const inputText = document.getElementById('inputText');
const result = document.getElementById('result');
const showResult = document.getElementById('showResult');

function generate() {
    const text = inputText.value;
    const invoke = window.__TAURI__.invoke;
    invoke('jproma', {text: text})
    .then((response) => {
        while(result.firstChild) {
            result.removeChild(result.firstChild);
        }
        let htmlResult = document.createTextNode(response);
        result.appendChild(htmlResult);
        showResult.innerHTML = response;
    });
}

generate();
//document.getElementById("submit").addEventListener("click", generate);
document.getElementById("inputText").addEventListener("keyup", generate);