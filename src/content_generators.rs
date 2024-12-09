pub fn generate_html_content(title: &str) -> String {
    format!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <title>{}</title>
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <link rel="icon" href="assets/favicon-white.png" />
    <link rel="stylesheet" href="styles/global.css" />
  </head>
  <body>
    <main></main>
    <script type="module" src="src/services/favicon-selector.js"></script>
    <script type="module" src="src/services/route-page.js"></script>
  </body>
</html>
"#,
        title
    )
}

pub fn generate_global_css_content() -> &'static str {
    r#"* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  --background: #1e201e;
  --code-block: #000000;
  --grey: #3c3d37;
  --light-grey: #697565;
  --muted-grey: #a9a9a9;
  --pink: #f4abc4;
  --light-pink: #ecdfcc;
  --yellow: #ffd369;
  --orange: #f09319;
  --light-blue: #6eacda;
  --light-green: #b1d690;
  --text: #ffffff;
}

html {
  font-family: Arial, Helvetica, sans-serif;
  font-optical-sizing: auto;
  font-weight: 400;
  font-size: 1rem;
  line-height: 1.5;
  font-style: normal;
}
@media (min-width: 640px) {
  html {
    font-size: 1.2rem;
  }
}

body {
  background-color: var(--background);
  color: var(--text);
  height: 100vh;
}
"#
}

pub fn generate_favicon_selector_content() -> &'static str {
    r#"const faviconLink = document.querySelector('link[rel="icon"]');

window
  .matchMedia("(prefers-color-scheme: dark)")
  .addEventListener("change", (event) => {
    if (event.matches) {
      faviconLink.href = "assets/favicon-white.png";
    } else {
      faviconLink.href = "assets/favicon-dark.png";
    }
  });

if (
  window.matchMedia &&
  window.matchMedia("(prefers-color-scheme: dark)").matches
) {
  faviconLink.href = "assets/favicon-white.png";
} else {
  faviconLink.href = "assets/favicon-dark.png";
}
"#
}

pub fn generate_component_js_content(
    component_name: &str,
    template_name: &str,
    class_name: &str,
) -> String {
    format!(
        r#"const {}Template = document.createElement("template");
{}Template.innerHTML = ``;

export class {} extends HTMLElement {{
  constructor() {{
    super();
  }}
}}

customElements.define("{}", {});
"#,
        template_name, template_name, class_name, component_name, class_name
    )
}
