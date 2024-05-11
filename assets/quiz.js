const username_regex = / @([^ ]+)/g;
document.querySelectorAll("h1")
    .forEach(h1 => {
        let usernames = [];
        for (let captures of h1.innerText.matchAll(username_regex)) {
            usernames.push(captures[1]);
        }
        h1.innerText = h1.innerText.replace(username_regex, "");

        let authors = document.createElement("div");
        authors.classList.add("quiz-authors");

        for (let username of usernames) {
            let img = document.createElement("img");
            img.classList.add("quiz-gh-avatar");
            img.src = "https://github.com/" + username + ".png";

            let a = document.createElement("a");
            a.classList.add("quiz-author");
            a.href = "https://github.com/" + username;
            a.innerText = username;
            a.prepend(img);

            authors.appendChild(a);
        }

        if (usernames.length > 0) {
            h1.appendChild(authors);
            h1.classList.add("quiz-with-authors");
        }
    });

// Make sure the <details> elements are open when printing.
window.matchMedia("print")
    .addEventListener("change", event => {
        if (event.matches) {
            document.body
                .querySelectorAll("details:not([open])")
                .forEach(e => e.setAttribute("open", ""));
        }
    });
