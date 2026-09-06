import Link from '@docusaurus/Link';
import {usePluginData} from '@docusaurus/useGlobalData';
import Layout from '@theme/Layout';
import Heading from '@theme/Heading';

import ArticleTree from '@site/src/components/ArticleTree';
import type {PortfolioMetadata} from '@site/src/data/portfolioMetadata';
import type {PortfolioProject} from '@site/src/data/projects';
import styles from '@site/src/pages/section.module.css';

type ProjectOverviewProps = {
  project: PortfolioProject;
};

export default function ProjectOverview({
  project,
}: ProjectOverviewProps): React.JSX.Element {
  const metadata = usePluginData('portfolio-metadata') as PortfolioMetadata;
  const articles = metadata.articles
    .filter(
      (article) =>
        article.collection === 'project' && article.project === project.slug,
    )
    .sort((left, right) => left.order - right.order);

  const facts = [
    ['기간', project.period],
    ['역할', project.role],
    ['참여 인원', project.teamSize],
    ['플랫폼', project.platform],
  ].filter((fact): fact is [string, string] => fact[1] !== undefined);

  return (
    <Layout title={project.title} description={project.description}>
      <main className={styles.page}>
        <div className={styles.content}>
          <Link className={styles.backLink} to="/projects">
            ← 프로젝트 목록
          </Link>
          <Heading as="h1" className={styles.title}>
            {project.title}
          </Heading>
          <dl className={styles.projectOverview} aria-label="프로젝트 기본 정보">
            {facts.map(([label, value]) => (
              <div className={styles.projectOverviewItem} key={label}>
                <dt>{label}</dt>
                <dd>{value}</dd>
              </div>
            ))}
          </dl>
          <p className={styles.description}>{project.description}</p>

          {project.overview && (
            <section className={styles.projectSection} aria-labelledby="overview-title">
              <Heading as="h2" id="overview-title" className={styles.sectionTitle}>
                프로젝트 개요
              </Heading>
              <p className={styles.projectIntro}>{project.overview}</p>
            </section>
          )}

          {project.tasks && project.tasks.length > 0 && (
            <section className={styles.projectSection} aria-labelledby="tasks-title">
              <Heading as="h2" id="tasks-title" className={styles.sectionTitle}>
                담당 작업
              </Heading>
              <div className={styles.taskGrid}>
                {project.tasks.map((task) => (
                  <article className={styles.taskCard} key={task.title}>
                    <Heading as="h3">{task.title}</Heading>
                    <p>{task.description}</p>
                  </article>
                ))}
              </div>
            </section>
          )}

          {project.collaboration && project.collaboration.length > 0 && (
            <section className={styles.projectSection} aria-labelledby="collaboration-title">
              <Heading as="h2" id="collaboration-title" className={styles.sectionTitle}>
                협업
              </Heading>
              <ul className={styles.projectList}>
                {project.collaboration.map((item) => (
                  <li key={item}>{item}</li>
                ))}
              </ul>
            </section>
          )}

          {articles.length > 0 && (
            <section className={styles.articleSection} aria-labelledby="articles-title">
              <Heading as="h2" id="articles-title" className={styles.sectionTitle}>
                기술 문서
              </Heading>
              <ArticleTree articles={articles} />
            </section>
          )}
        </div>
      </main>
    </Layout>
  );
}
