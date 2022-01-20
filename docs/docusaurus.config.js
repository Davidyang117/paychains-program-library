module.exports = {
  title: "PayChains Program Library Docs",
  tagline:
    "PayChains is an open source project implementing a new, high-performance, permissionless blockchain.",
  url: "https://spl.paychains.com",
  baseUrl: "/",
  favicon: "img/favicon.ico",
  organizationName: "paychains-labs", // Usually your GitHub org/user name.
  projectName: "paychains-program-library", // Usually your repo name.
  themeConfig: {
    navbar: {
      logo: {
        alt: "PayChains Logo",
        src: "img/logo-horizontal.svg",
        srcDark: "img/logo-horizontal-dark.svg",
      },
      links: [
        {
          href: "https://docs.paychains.com/",
          label: "Docs »",
          position: "left",
        },
        {
          href: "https://discordapp.com/invite/pquxPsq",
          label: "Chat",
          position: "right",
        },

        {
          href: "https://github.com/paychains-labs/paychains-program-library",
          label: "GitHub",
          position: "right",
        },
      ],
    },
    footer: {
      style: "dark",
      links: [
        {
          title: "Community",
          items: [
            {
              label: "Discord",
              href: "https://discordapp.com/invite/pquxPsq",
            },
            {
              label: "Twitter",
              href: "https://twitter.com/paychains",
            },
            {
              label: "Forums",
              href: "https://forums.paychains.com",
            },
          ],
        },
        {
          title: "More",
          items: [
            {
              label: "GitHub",
              href: "https://github.com/paychains-labs/paychains-program-library",
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} PayChains Foundation`,
    },
  },
  plugins: [require.resolve('docusaurus-lunr-search')],
  presets: [
    [
      "@docusaurus/preset-classic",
      {
        docs: {
          path: "src",
          routeBasePath: "/",
          homePageId: "introduction",
          sidebarPath: require.resolve("./sidebars.js"),
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      },
    ],
  ],
};
