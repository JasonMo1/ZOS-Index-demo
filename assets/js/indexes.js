var urls = {
    "offical": "https://zeal8bit.com/roms/index.json",
    "demo": "https://jasonmo1.github.io/ZOS-Index-demo/index.json",
    // "test": "https://github.com/JasonMo1/ZOS-Index-demo/releases/download/v0.1.0/index.json",
};

async function load_indexes() {
    let to_code = entry =>
    `<h3>${entry[0][0].toUpperCase()+entry[0].substr(1)} Index</h3>
    <button onclick="download_index(${"\'"+entry[0]+"\'"});">Download this index</button>
    <div class="language-plaintext highlighter-rouge">
      <div class="highlight">
        <pre class="highlight">
          <code>${entry[1]}</code>
        </pre>
      </div>
    </div>`

    for (var this_url in urls) {
        try {
            let index_text = await fetch(urls[this_url])
                .then(res => res.text())
                .then(res => "\n"+res);
            let index_name = this_url;
            let index = [index_name, index_text];
            let index_html = to_code(index);
            document.getElementById('index-container').innerHTML += index_html;
        }
        catch (error) {
            console.error(error);
            continue;
        }
    }
}

async function load_image_select() {
    for (var this_url in urls) {
        try {
            let index = await fetch(urls[this_url]).then(res => res.json());
            process_index(index);
        } catch (error) {
            console.error(error);
            continue;
        }
    }
}

async function download_index(index) {
    let text = await fetch(urls[index]).then(res => res.text());
    save_text(text, "index.json");
}

function download_image() {
    let url = $("#image_select").val();
    let name = $("#image_select").text();
    save_url(url, name+".img");
}

function process_index(index) {
    const to_option = entry => `<option value="${entry.urls}" data-version="${entry.version}" data-hash="${entry.hash}">${entry.name}</option>`;
    const to_separator = entry => `<option value="" disabled>--- ${entry[0].toUpperCase()+entry.substr(1)} ---</option>`;
    var all_options;

    /* Generate an HTML option out of each entry */
    for (var key in index) {
        if (typeof index[key] == "object") {
            let this_option = to_separator(key);
            if (typeof index[key][0] == "object") {
                this_option += Object.values(index[key]).map(to_option).join("");
            }
            else {
                this_option += to_option(index[key]);
            }
            all_options += this_option;
        }
    }
    document.getElementById('image_select').innerHTML += all_options;
}
