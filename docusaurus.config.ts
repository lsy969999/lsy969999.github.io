import {readdirSync} from 'node:fs';
import {join} from 'node:path';

import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

function containsMarkdown(directory: string): boolean {
  try {
    return readdirSync(directory, {withFileTypes: true}).some((entry) => {
      const path = join(directory, entry.name);
      return entry.isDirectory()
        ? containsMarkdown(path)
        : /\.mdx?$/.test(entry.name);
    });
  } catch (error) {
    const code = (error as NodeJS.ErrnoException).code;
    if (code === 'ENOENT') {
      return false;
    }
    throw error;
  }
}

const projectDocsExist = containsMarkdown(join(__dirname, 'projects'));
const archiveDocsExist = containsMarkdown(join(__dirname, 'archive'));

const config: Config = {
  title: 'Limits1214 | Portfolio',
  tagline: 'C++ Game Client & Engine Programmer',
  favicon: 'img/favicon.ico',

  url: 'https://limits1214.github.io',
  baseUrl: '/',
  organizationName: 'limits1214',
  projectName: 'limits1214.github.io',

  onBrokenLinks: 'throw',
  markdown: {
    hooks: {
      onBrokenMarkdownLinks: 'warn',
    },
  },

  i18n: {
    defaultLocale: 'ko',
    locales: ['ko'],
  },

  future: {
    v4: true,
    faster: {
      rspackPersistentCache: false,
    },
  },

  presets: [
    [
      'classic',
      {
        docs: projectDocsExist
          ? {
              path: 'projects',
              routeBasePath: 'projects',
              sidebarPath: false,
              breadcrumbs: false,
              showLastUpdateAuthor: false,
              showLastUpdateTime: false,
            }
          : false,
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  plugins: [
    './portfolio-metadata-plugin.mjs',
    archiveDocsExist && [
      '@docusaurus/plugin-content-docs',
      {
        id: 'archive',
        path: 'archive',
        routeBasePath: 'archive',
        sidebarPath: false,
        breadcrumbs: false,
        showLastUpdateAuthor: false,
        showLastUpdateTime: false,
      },
    ],
  ],

  themeConfig: {
    navbar: {
      title: 'Limits1214',
      items: [
        {to: '/', label: 'Home', position: 'left'},
        {to: '/projects', label: 'Projects', position: 'left'},
        {to: '/archive', label: 'Archive', position: 'left'},
        ...(archiveDocsExist
          ? [{to: '/archive/tags', label: 'Tags', position: 'left' as const}]
          : []),
        {to: '/about', label: 'About', position: 'left'},
        {
          href: 'https://github.com/limits1214',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      copyright: `Copyright © ${new Date().getFullYear()} Limits1214. Built with Docusaurus.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
