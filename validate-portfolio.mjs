import {dirname} from 'node:path';
import {fileURLToPath} from 'node:url';

import portfolioMetadataPlugin from './portfolio-metadata-plugin.mjs';

const siteDir = dirname(fileURLToPath(import.meta.url));
const content = await portfolioMetadataPlugin({siteDir}).loadContent();
const projectArticleCount = content.articles.filter(
  (article) => article.collection === 'project',
).length;
const archiveArticleCount = content.articles.filter(
  (article) => article.collection === 'archive',
).length;

console.log(
  `[Portfolio] Validation passed: ${content.projects.length} projects, ` +
    `${projectArticleCount} project articles, ${archiveArticleCount} archive articles.`,
);
