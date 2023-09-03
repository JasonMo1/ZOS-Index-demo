function save_text(text, fileName) {
    let downloadLink = document.getElementById("fordown");
    downloadLink.download = fileName;
    downloadLink.href = "data:text/plain;charset=utf-8,"+encodeURIComponent(text);
    $("body").append(downloadLink)
    downloadLink.click();
}

function save_url(url, fileName) {
    let downloadLink = document.getElementById("fordown");
    downloadLink.download = fileName;
    downloadLink.href = url;
    $("body").append(downloadLink)
    downloadLink.click();
}