async function load_indexes() {
    let index_offical_blob = await fetch("https://zeal8bit.com/roms/index.json");
    let index_offical_text = await index_offical_blob.text();
    let index_offical = "\n"+index_offical_text;
    document.getElementById('zos-index-offical').textContent = index_offical;
    let index_demo_blob = await fetch("https://jasonmo1.github.io/ZOS-Index-demo/index.json");
    let index_demo_text = await index_demo_blob.text();
    let index_demo = "\n"+index_demo_text;
    document.getElementById('zos-index-demo').textContent = index_demo;
}

async function download_index(index) {
    let url;
    let text;
    switch(index) {
        case "offical":
            url = "https://zeal8bit.com/roms/index.json";
            index_blob = await fetch(url);
            text = await index_blob.text();
            break;
        case "demo":
            url = "https://jasonmo1.github.io/ZOS-Index-demo/index.json";
            index_blob = await fetch(url);
            text = await index_blob.text();
            break;
    }
    saveCodeAsFile(text, "index.json");
}

function saveCodeAsFile(text, progname) {
    let downloadLink = document.getElementById("fordown");
    downloadLink.download = progname;
    downloadLink.href = "data:text/plain;charset=utf-8,"+encodeURIComponent(text);
    downloadLink.click();
}

// function download(url, filename) {
//     let a = document.createElement('a');
//     a.href = url;
//     a.download = filename;
//     a.click()
// }
