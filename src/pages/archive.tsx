import {usePluginData} from '@docusaurus/useGlobalData';
import Layout from '@theme/Layout';
import Heading from '@theme/Heading';

import ArticleTree from '@site/src/components/ArticleTree';
import type {PortfolioMetadata} from '@site/src/data/portfolioMetadata';
import styles from './section.module.css';

export default function Archive(): React.JSX.Element {
  const metadata = usePluginData('portfolio-metadata') as PortfolioMetadata;
  const articles = metadata.articles
    .filter((article) => article.collection === 'archive')
    .sort((left, right) => {
      if (left.featured !== right.featured) {
        return left.featured ? -1 : 1;
      }
      return left.order - right.order;
    });

  return (
    <Layout
      title="Archive"
      description="기술 분야별 개발 기록과 문제 해결 아카이브">
      <main className={styles.page}>
        <div className={styles.content}>
          <Heading as="h1" className={styles.title}>
            Archive
          </Heading>
          <p className={styles.description}>
            프로젝트 경계를 넘어 기술 분야별 개발 기록을 찾아볼 수 있는
            공간입니다.
          </p>

          <section className={styles.articleSection} aria-labelledby="archive-articles-title">
            <div className={styles.archiveSectionHeader}>
              <Heading as="h2" id="archive-articles-title" className={styles.sectionTitle}>
                Articles
              </Heading>
              <span className={styles.articleCount}>
                {articles.length} documents
              </span>
            </div>

            {articles.length > 0 ? (
              <ArticleTree articles={articles} />
            ) : (
              <p className={styles.emptyState}>
                등록된 기술 문서가 없습니다. archive 폴더에 MDX를 추가해 주세요.
              </p>
            )}
          </section>
        </div>
      </main>
    </Layout>
  );
}
