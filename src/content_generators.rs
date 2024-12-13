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
    <script type="module" src="src/services/router.js"></script>
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

pub fn generate_page_element_class_content() -> &'static str {
    r#"export class PageElement extends HTMLElement {
  /**
   * @param {HTMLTemplateElement} template
   */
  constructor(template) {
    super();
    template.innerHTML = "<slot><slot>";
    this.attachShadow({ mode: "open" });
    this.shadowRoot.appendChild(template.content.cloneNode(true));
    const slot = this.shadowRoot.querySelector("slot");
    /**
     * @type {Node[]}
     * @public
     */
    this.nodes = slot
      .assignedNodes()
      .filter((n) => n.nodeType === Node.ELEMENT_NODE);
  }

  /**
   * @param {string} tagName
   * @returns {Node | undefined}
   */
  getSlottedElementByTagName(tagName) {
    return this.nodes.find((n) => n.localName === tagName);
  }

  /**
   * @param {string} tagName
   * @returns {Node[]}
   */
  getAllSlottedElementByTagName(tagName) {
    return this.nodes.filter((n) => n.localName === tagName);
  }

  /**
   * @param {string} key
   * @param {string} value
   * @returns {Node[]}
   */
  getSlottedElementByDataset(key, value) {
    return this.nodes.find((n) => n.dataset[key] === value);
  }

  /**
   * @param {string} key
   * @param {string} value
   * @returns {Node[]}
   */
  getAllSlottedElementByDataset(key, value) {
    return this.nodes.filter((n) => n.dataset[key] === value);
  }
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
    this.attachShadow({{ mode: 'open' }});
    this.shadowRoot.appendChild({}Template.content.cloneNode(true));
  }}
}}

customElements.define("{}", {});
"#,
        template_name, template_name, class_name, template_name, component_name, class_name
    )
}

pub fn generate_page_component_js_content(
    component_name: &str,
    template_name: &str,
    class_name: &str,
) -> String {
    format!(
        r#"import {{ PageElement }} from "../../services/shadow-dom.js";

const {}Template = document.createElement("template");

export class {} extends PageElement {{
  constructor() {{
    super({}Template);
  }}
}}

customElements.define("{}", {});
"#,
        template_name, class_name, template_name, component_name, class_name
    )
}

pub fn generate_home_html_content() -> &'static str {
    r#"<h1>Hello from scaff</h1>
"#
}

pub fn generate_home_css_content() -> &'static str {
    r#":slotted(h1) {
  color: red;
}
"#
}

pub fn generate_router_js_content() -> &'static str {
    r#"const main = document.querySelector("main");

async function getPageFragment(route) {
  const normalizedRoute = route === "/" ? "home" : route.replace(/^\//, "");
  const response = await fetch(`/api/page/${normalizedRoute}`);
  if (!response.ok) {
    throw new Error(`Failed to load page: ${response.statusText}`);
  }
  return response.text();
}

async function updatePage(route) {
  try {
    changePageScript(route);
    const html = await getPageFragment(route);
    main.innerHTML = html;
    history.pushState(route, "", route);
  } catch (error) {
    console.error("Error loading page:", error);
  }
}

function changePageScript(route) {
  const pageScriptId = "page-script";
  const normalizedRoute = route === "/" ? "home" : route.replace(/^\//, "");
  const scriptSrc = `src/page/${normalizedRoute}/${normalizedRoute}.js`;
  const scriptTag = document.getElementById(pageScriptId);
  if (scriptTag) {
    scriptTag.remove();
  }
  const script = document.createElement("script");
  script.type = "module";
  script.id = pageScriptId;
  script.src = scriptSrc;
  document.body.appendChild(script);
}

window.addEventListener("DOMContentLoaded", () => {
  updatePage(window.location.pathname);
});

window.addEventListener("popstate", (e) => {
  const route = e.state || "/";
  updatePage(route);
});

document.addEventListener("click", (e) => {
  const link = e.target.closest("a");
  if (!link) return;

  const url = new URL(link.href);
  if (url.origin === window.location.origin) {
    e.preventDefault();
    if (url.pathname === window.location.pathname) return;
    updatePage(url.pathname);
  }
});
"#
}
