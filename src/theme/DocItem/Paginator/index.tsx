import {useDoc} from '@docusaurus/plugin-content-docs/client';
import {usePluginData} from '@docusaurus/useGlobalData';
import DocPaginator from '@theme/DocPaginator';

import type {
  PortfolioArticle,
  PortfolioMetadata,
} from '@site/src/data/portfolioMetadata';

function compareNavigationOrder(
  left: PortfolioArticle,
  right: PortfolioArticle,
): number {
  const groupCount = Math.max(left.groupPath.length, right.groupPath.length);

  for (let index = 0; index < groupCount; ++index) {
    const leftGroup = left.groupPath[index];
    const rightGroup = right.groupPath[index];

    if (!leftGroup) {
      return -1;
    }
    if (!rightGroup) {
      return 1;
    }

    const groupOrder = leftGroup.position - rightGroup.position ||
      leftGroup.label.localeCompare(rightGroup.label, 'ko');
    if (groupOrder !== 0) {
      return groupOrder;
    }
  }

  return left.order - right.order || left.title.localeCompare(right.title, 'ko');
}

export default function DocItemPaginator(): React.JSX.Element | null {
  const {metadata} = useDoc();
  const portfolioMetadata = usePluginData(
    'portfolio-metadata',
  ) as PortfolioMetadata;
  const currentArticle = portfolioMetadata.articles.find(
    (article) => article.permalink === metadata.permalink,
  );

  if (!currentArticle) {
    return null;
  }

  const navigationArticles = portfolioMetadata.articles
    .filter((article) => {
      if (article.collection !== currentArticle.collection) {
        return false;
      }

      return currentArticle.collection === 'project'
        ? article.project === currentArticle.project
        : true;
    })
    .sort(compareNavigationOrder);
  const currentIndex = navigationArticles.findIndex(
    (article) => article.permalink === currentArticle.permalink,
  );
  const previous = navigationArticles[currentIndex - 1];
  const next = navigationArticles[currentIndex + 1];

  if (!previous && !next) {
    return null;
  }

  return (
    <DocPaginator
      className="portfolio-pagination docusaurus-mt-lg"
      previous={previous && {
        title: previous.title,
        permalink: previous.permalink,
      }}
      next={next && {
        title: next.title,
        permalink: next.permalink,
      }}
    />
  );
}
