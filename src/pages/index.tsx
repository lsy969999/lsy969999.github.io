import Link from '@docusaurus/Link';
import Layout from '@theme/Layout';
import Heading from '@theme/Heading';

import styles from './index.module.css';

export default function Home(): React.JSX.Element {
  return (
    <Layout
      title="Portfolio"
      description="Limits1214의 게임 클라이언트·엔진 개발 포트폴리오">
      <main className={styles.page}>
        <section className={styles.hero}>
          <p className={styles.eyebrow}>C++ GAME CLIENT · ENGINE</p>
          <Heading as="h1" className={styles.title}>
            Limits1214
          </Heading>
          <p className={styles.description}>
            포트폴리오를 새로운 구조로 구성하고 있습니다.
          </p>
          <div className={styles.actions}>
            <Link
              className="button button--primary button--lg"
              to="/projects">
              Projects
            </Link>
            <Link
              className="button button--secondary button--lg"
              href="https://github.com/limits1214">
              GitHub
            </Link>
          </div>
        </section>
      </main>
    </Layout>
  );
}
