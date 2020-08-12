function convert(buffer) {
    const u8array = new Uint8Array(buffer)
    const plainBuffer = u8array.map(u=>u^0xff);
    return new TextDecoder("utf-8").decode(plainBuffer);
}

function getFile(form) {
    const formData = new FormData(form);
    return formData.get('config-file');
}

function getCipher(file) {
    const reader = new FileReader();
    reader.onload = function (e) {
        const cipherBuffer = e.target.result;
        const plainText = convert(cipherBuffer);
        code.innerHTML = formatXml(plainText);
    }
    reader.readAsArrayBuffer(file);
}

function formatXml(string) {
   return  Prism.highlight(string, Prism.languages.xml, 'xml');
}

function onSubmit(e) {
    e.preventDefault();
    const file = getFile(e.target);
    if (!file || file.size < 1) {
        alert('请先选择文件');
        return;
    }
    getCipher(file);
    // get file
    // convert file
    // format xml
}

const form = document.getElementById('config-form');
form.addEventListener('submit', onSubmit);
const code = document.getElementById('result');