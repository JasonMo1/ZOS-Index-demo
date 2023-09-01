async function load_readme() {
    anchors.add();
    let readme_blob = await fetch("https://jasonmo1.github.io/ZOS-Index-demo/README.md");
    let readme_text = await readme_blob.text();
    document.getElementById('content').innerHTML = marked.parse(readme_text);
}
