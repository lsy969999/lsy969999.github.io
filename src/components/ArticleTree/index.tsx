import Link from '@docusaurus/Link';

import type {
  PortfolioArticle,
  PortfolioGroup,
} from '@site/src/data/portfolioMetadata';
import styles from '@site/src/pages/section.module.css';

type ArticleTreeProps = {
  articles: PortfolioArticle[];
};

type ArticleGroupNode = PortfolioGroup & {
  key: string;
  articles: PortfolioArticle[];
  children: ArticleGroupNode[];
};

type MutableArticleGroupNode = PortfolioGroup & {
  key: string;
  articles: PortfolioArticle[];
  children: Map<string, MutableArticleGroupNode>;
};

function compareArticles(left: PortfolioArticle, right: PortfolioArticle): number {
  if (left.featured !== right.featured) {
    return left.featured ? -1 : 1;
  }
  return left.order - right.order || left.title.localeCompare(right.title, 'ko');
}

function compareGroups(left: ArticleGroupNode, right: ArticleGroupNode): number {
  return left.position - right.position || left.label.localeCompare(right.label, 'ko');
}

function finalizeGroup(node: MutableArticleGroupNode): ArticleGroupNode {
  return {
    id: node.id,
    key: node.key,
    label: node.label,
    position: node.position,
    collapsed: node.collapsed,
    articles: [...node.articles].sort(compareArticles),
    children: Array.from(node.children.values())
      .map(finalizeGroup)
      .sort(compareGroups),
  };
}

function buildArticleTree(articles: PortfolioArticle[]): ArticleGroupNode {
  const root: MutableArticleGroupNode = {
    id: 'root',
    key: 'root',
    label: 'root',
    position: 0,
    collapsed: false,
    articles: [],
    children: new Map(),
  };

  for (const article of articles) {
    let current = root;
    let groupKey = '';

    for (const group of article.groupPath) {
      groupKey = groupKey ? `${groupKey}/${group.id}` : group.id;
      let child = current.children.get(group.id);

      if (!child) {
        child = {
          ...group,
          key: groupKey,
          articles: [],
          children: new Map(),
        };
        current.children.set(group.id, child);
      }

      current = child;
    }

    current.articles.push(article);
  }

  return finalizeGroup(root);
}

function countGroupArticles(group: ArticleGroupNode): number {
  return group.articles.length +
    group.children.reduce((total, child) => total + countGroupArticles(child), 0);
}

function FolderIcon(): React.JSX.Element {
  return (
    <span className={styles.treeFolderIcon} aria-hidden="true">
      <svg
        className={styles.treeFolderClosed}
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="1.8"
        strokeLinecap="round"
        strokeLinejoin="round">
        <path d="M3 6.5h6l2 2h10v10H3z" />
        <path d="M3 6.5V5h7l2 2" />
      </svg>
      <svg
        className={styles.treeFolderOpen}
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="1.8"
        strokeLinecap="round"
        strokeLinejoin="round">
        <path d="M3 17.5V6h6l2 2h9v3" />
        <path d="M5 10.5h16.5l-3 8H2.5z" />
      </svg>
    </span>
  );
}

function ArticleLeaf({article}: {article: PortfolioArticle}): React.JSX.Element {
  return (
    <li className={styles.treeDocument} role="treeitem">
      <Link className={styles.treeDocumentLink} to={article.permalink}>
        <span className={styles.treeDocumentIcon} aria-hidden="true" />
        <span className={styles.treeDocumentBody}>
          <span className={styles.treeDocumentTitle}>{article.title}</span>
          <span className={styles.treeDocumentDescription}>
            {article.description}
          </span>
          <span className={styles.treeDocumentTags}>
            {article.tags.map((tag) => (
              <span key={tag}>{tag}</span>
            ))}
          </span>
        </span>
      </Link>
    </li>
  );
}

function GroupNode({
  group,
}: {
  group: ArticleGroupNode;
}): React.JSX.Element {
  return (
    <li className={styles.treeGroup} role="treeitem">
      <details open={!group.collapsed}>
        <summary className={styles.treeGroupSummary}>
          <FolderIcon />
          <span className={styles.treeGroupLabel}>{group.label}</span>
          <span className={styles.treeGroupCount}>{countGroupArticles(group)}</span>
        </summary>
        <ul className={styles.treeChildren} role="group">
          {group.articles.map((article) => (
            <ArticleLeaf
              key={`${article.collection}-${article.id}`}
              article={article}
            />
          ))}
          {group.children.map((child) => (
            <GroupNode key={child.key} group={child} />
          ))}
        </ul>
      </details>
    </li>
  );
}

export default function ArticleTree({
  articles,
}: ArticleTreeProps): React.JSX.Element {
  const tree = buildArticleTree(articles);

  return (
    <ul className={styles.articleTree} role="tree">
      {tree.articles.map((article) => (
        <ArticleLeaf
          key={`${article.collection}-${article.id}`}
          article={article}
        />
      ))}
      {tree.children.map((group) => (
        <GroupNode key={group.key} group={group} />
      ))}
    </ul>
  );
}
