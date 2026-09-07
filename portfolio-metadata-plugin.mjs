import {readdir, readFile} from 'node:fs/promises';
import {dirname, extname, join, relative} from 'node:path';

import matter from 'gray-matter';

async function findMdxFiles(directory) {
  const entries = await readdir(directory, {withFileTypes: true});
  const nested = await Promise.all(
    entries.map(async (entry) => {
      const path = join(directory, entry.name);
      if (entry.isDirectory()) {
        return findMdxFiles(path);
      }
      const extension = extname(entry.name);
      return extension === '.md' || extension === '.mdx' ? [path] : [];
    }),
  );
  return nested.flat();
}

function requireString(frontMatter, field, source) {
  const value = frontMatter[field];
  if (typeof value !== 'string' || value.trim().length === 0) {
    throw new Error(`[Portfolio] ${source}: '${field}' front matter가 필요합니다.`);
  }
  return value;
}

function optionalString(value, field, source) {
  if (value === undefined) {
    return undefined;
  }
  if (typeof value !== 'string' || value.trim().length === 0) {
    throw new Error(`[Portfolio] ${source}: '${field}'는 비어 있지 않은 문자열이어야 합니다.`);
  }
  return value;
}

function optionalStringArray(value, field, source) {
  if (value === undefined) {
    return undefined;
  }
  if (
    !Array.isArray(value) ||
    value.some((item) => typeof item !== 'string' || item.trim().length === 0)
  ) {
    throw new Error(`[Portfolio] ${source}: '${field}'는 비어 있지 않은 문자열 배열이어야 합니다.`);
  }
  return value;
}

function optionalTasks(value, source) {
  if (value === undefined) {
    return undefined;
  }
  if (!Array.isArray(value)) {
    throw new Error(`[Portfolio] ${source}: 'tasks'는 배열이어야 합니다.`);
  }

  return value.map((task, index) => {
    const itemSource = `${source}.tasks[${index}]`;
    if (typeof task !== 'object' || task === null) {
      throw new Error(`[Portfolio] ${itemSource}: 객체가 필요합니다.`);
    }

    return {
      title: requireString(task, 'title', itemSource),
      description: requireString(task, 'description', itemSource),
    };
  });
}

function loadProjectDefinitions(value, source) {
  if (!Array.isArray(value)) {
    throw new Error(`[Portfolio] ${source}: 프로젝트 목록은 배열이어야 합니다.`);
  }

  const slugs = new Set();
  return value.map((project, index) => {
    const itemSource = `${source}[${index}]`;
    const slug = requireString(project, 'slug', itemSource);

    if ('responsibilities' in project || 'highlights' in project) {
      throw new Error(`[Portfolio] ${itemSource}: 담당 작업은 'tasks'에 통합해 작성하세요.`);
    }

    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(slug)) {
      throw new Error(
        `[Portfolio] ${itemSource}: slug '${slug}'는 소문자, 숫자와 하이픈만 사용할 수 있습니다.`,
      );
    }
    if (slugs.has(slug)) {
      throw new Error(`[Portfolio] ${source}: 중복된 project slug '${slug}'입니다.`);
    }
    slugs.add(slug);

    return {
      slug,
      title: requireString(project, 'title', itemSource),
      period: requireString(project, 'period', itemSource),
      description: requireString(project, 'description', itemSource),
      role: optionalString(project.role, 'role', itemSource),
      teamSize: optionalString(project.teamSize, 'teamSize', itemSource),
      platform: optionalString(project.platform, 'platform', itemSource),
      githubUrl: optionalString(project.githubUrl, 'githubUrl', itemSource),
      overview: optionalString(project.overview, 'overview', itemSource),
      collaboration: optionalStringArray(project.collaboration, 'collaboration', itemSource),
      tasks: optionalTasks(project.tasks, itemSource),
    };
  });
}

function validateCategory(category, source) {
  if (
    category.label !== undefined &&
    (typeof category.label !== 'string' || category.label.trim().length === 0)
  ) {
    throw new Error(`[Portfolio] ${source}: label은 비어 있지 않은 문자열이어야 합니다.`);
  }
  if (
    category.position !== undefined &&
    (typeof category.position !== 'number' || !Number.isFinite(category.position))
  ) {
    throw new Error(`[Portfolio] ${source}: position은 유한한 숫자여야 합니다.`);
  }
  if (category.collapsed !== undefined && typeof category.collapsed !== 'boolean') {
    throw new Error(`[Portfolio] ${source}: collapsed는 boolean이어야 합니다.`);
  }
}

function validateArticles(articles) {
  const ids = new Set();
  const permalinks = new Set();

  for (const article of articles) {
    const id = `${article.collection}:${article.id}`;
    if (ids.has(id)) {
      throw new Error(`[Portfolio] 중복된 문서 id '${id}'입니다.`);
    }
    if (permalinks.has(article.permalink)) {
      throw new Error(`[Portfolio] 중복된 문서 경로 '${article.permalink}'입니다.`);
    }
    ids.add(id);
    permalinks.add(article.permalink);
  }
}

function formatDirectoryLabel(directoryName) {
  return directoryName
    .split('-')
    .filter(Boolean)
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
}

async function loadGroupPath(groupRoot, groupDirectories, categoryCache) {
  const groups = [];
  let currentDirectory = groupRoot;

  for (const directoryName of groupDirectories) {
    currentDirectory = join(currentDirectory, directoryName);
    const categoryPath = join(currentDirectory, '_category_.json');
    let category = categoryCache.get(categoryPath);

    if (!category) {
      try {
        category = JSON.parse(await readFile(categoryPath, 'utf8'));
        validateCategory(category, categoryPath);
      } catch (error) {
        if (error.code !== 'ENOENT') {
          throw new Error(`[Portfolio] ${categoryPath}: _category_.json을 읽지 못했습니다.`, {
            cause: error,
          });
        }
        category = {};
      }
      categoryCache.set(categoryPath, category);
    }

    groups.push({
      id: directoryName,
      label:
        typeof category.label === 'string' && category.label.trim().length > 0
          ? category.label
          : formatDirectoryLabel(directoryName),
      position: typeof category.position === 'number' ? category.position : 999,
      collapsed: category.collapsed === true,
    });
  }

  return groups;
}

export default function portfolioMetadataPlugin(context) {
  const projectsPath = join(context.siteDir, 'src', 'data', 'projects.json');
  const contentSources = [
    {
      collection: 'project',
      contentPath: join(context.siteDir, 'projects'),
      routeBasePath: '/projects',
    },
    {
      collection: 'archive',
      contentPath: join(context.siteDir, 'archive'),
      routeBasePath: '/archive',
    },
  ];

  return {
    name: 'portfolio-metadata',

    getPathsToWatch() {
      return [
        projectsPath,
        ...contentSources.flatMap(({contentPath}) => [
          contentPath,
          join(contentPath, '**', '*.{md,mdx,json}'),
        ]),
      ];
    },

    async loadContent() {
      const projects = loadProjectDefinitions(
        JSON.parse(await readFile(projectsPath, 'utf8')),
        relative(context.siteDir, projectsPath).replaceAll('\\', '/'),
      );
      const projectSlugs = new Set(projects.map((project) => project.slug));
      const categoryCache = new Map();
      const sourceFiles = await Promise.all(
        contentSources.map(async (contentSource) => ({
          ...contentSource,
          files: await findMdxFiles(contentSource.contentPath),
        })),
      );
      const loadedArticles = await Promise.all(
        sourceFiles.flatMap((contentSource) =>
          contentSource.files.map(async (file) => {
            const source = relative(context.siteDir, file).replaceAll('\\', '/');
            const {data} = matter(await readFile(file, 'utf8'));

            // Docusaurus 문서 플러그인과 동일하게 draft 문서는 개발 환경에서만 노출한다.
            if (data.draft === true && process.env.NODE_ENV === 'production') {
              return null;
            }

            const slug = requireString(data, 'slug', source);
            const project =
              contentSource.collection === 'project'
                ? requireString(data, 'project', source)
                : undefined;

            if (project !== undefined && !projectSlugs.has(project)) {
              throw new Error(
                `[Portfolio] ${source}: 등록되지 않은 project '${project}'입니다.`,
              );
            }
            const relativeDirectory = relative(
              contentSource.contentPath,
              dirname(file),
            );
            const directorySegments = relativeDirectory === ''
              ? []
              : relativeDirectory.split(/[\\/]+/);
            const groupDirectories = contentSource.collection === 'project'
              ? directorySegments.slice(1)
              : directorySegments;
            const groupRoot = contentSource.collection === 'project'
              ? join(contentSource.contentPath, project)
              : contentSource.contentPath;

            if (
              contentSource.collection === 'project' &&
              directorySegments[0] !== project
            ) {
              throw new Error(
                `[Portfolio] ${source}: project '${project}' 문서는 같은 이름의 프로젝트 폴더 안에 있어야 합니다.`,
              );
            }

            const tags = Array.isArray(data.tags)
              ? data.tags.filter((tag) => typeof tag === 'string')
              : [];

            return {
              id: requireString(data, 'id', source),
              collection: contentSource.collection,
              project,
              title: requireString(data, 'title', source),
              description: requireString(data, 'description', source),
              permalink: `${contentSource.routeBasePath}${slug}`,
              order: typeof data.order === 'number' ? data.order : 999,
              featured: data.featured === true,
              tags,
              groupPath: await loadGroupPath(
                groupRoot,
                groupDirectories,
                categoryCache,
              ),
            };
          }),
        ),
      );
      const articles = loadedArticles.filter((article) => article !== null);
      validateArticles(articles);

      return {
        articles: articles.sort((left, right) => left.order - right.order),
        projects,
      };
    },

    async contentLoaded({content, actions}) {
      actions.setGlobalData({articles: content.articles});

      for (const project of content.projects) {
        actions.addRoute({
          path: `/projects/${project.slug}`,
          component: join(
            context.siteDir,
            'src',
            'components',
            'ProjectOverview',
            'index.tsx',
          ),
          exact: true,
          props: {project},
        });
      }
    },
  };
}
