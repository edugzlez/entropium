import { defineConfig } from "vitepress";

export default defineConfig({
  title: "entropium",
  description:
    "Information-theory primitives for Rust: entropy, joint entropy, conditional entropy, and mutual information.",
  base: "/entropium/",

  head: [["link", { rel: "icon", href: "/entropium/favicon.svg" }]],

  themeConfig: {
    nav: [
      { text: "Guide", link: "/guide/getting-started" },
      { text: "API", link: "/api/" },
      {
        text: "Changelog",
        link: "https://github.com/edugzlez/entropium/blob/master/CHANGELOG.md",
      },
    ],

    sidebar: {
      "/guide/": [
        {
          text: "Guide",
          items: [{ text: "Getting Started", link: "/guide/getting-started" }],
        },
      ],
      "/api/": [
        {
          text: "API Reference",
          items: [
            { text: "Overview", link: "/api/" },
            { text: "entropy", link: "/api/entropy" },
            { text: "joint_entropy", link: "/api/joint-entropy" },
            { text: "conditional_entropy", link: "/api/conditional-entropy" },
            { text: "mutual_information", link: "/api/mutual-information" },
            { text: "kl_divergence", link: "/api/kl-divergence" },
            { text: "js_divergence", link: "/api/js-divergence" },
            { text: "cross_entropy", link: "/api/cross-entropy" },
          ],
        },
      ],
    },

    socialLinks: [
      { icon: "github", link: "https://github.com/edugzlez/entropium" },
    ],

    footer: {
      message: "Released under the MIT OR Apache-2.0 License.",
    },

    search: {
      provider: "local",
    },
  },

  markdown: {
    math: true,
  },
});
