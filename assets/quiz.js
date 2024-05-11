// Make sure the <details> elements are open when printing.
window.matchMedia("print")
    .addEventListener("change", event => {
        if (event.matches) {
            document.body
                .querySelectorAll("details:not([open])")
                .forEach(e => e.setAttribute("open", ""));
        }
    });
