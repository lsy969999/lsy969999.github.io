import Layout from '@theme/Layout';
import Heading from '@theme/Heading';

import CareerTimeline from '@site/src/components/CareerTimeline';
import styles from './section.module.css';

export default function About(): React.JSX.Element {
  return (
    <Layout title="About" description="Limits1214 개발자 소개">
      <main className={styles.page}>
        <div className={styles.content}>
          <Heading as="h1" className={styles.title}>
            About
          </Heading>
          <p className={styles.description}>
            2020년 KOSMO JAVA Backend 과정을 시작으로 2020년 10월부터 2024년
            1월까지 실무 경험을 쌓았습니다. 한국방송통신대학교 컴퓨터과학과에
            편입·졸업했으며, 이후 C++ 기반 게임 클라이언트와 엔진 개발로
            영역을 확장했습니다.
          </p>

          <section aria-labelledby="journey-title">
            <Heading as="h2" id="journey-title" className={styles.sectionTitle}>
              Journey
            </Heading>
            <CareerTimeline />
          </section>

          <section className={styles.projectSection} aria-labelledby="qualifications-title">
            <Heading as="h2" id="qualifications-title" className={styles.sectionTitle}>
              자격증
            </Heading>
            <ul className={styles.projectList}>
              <li>정보처리기사 · <time dateTime="2025-12-24">2025.12.24</time></li>
              <li>정보처리산업기사 · <time dateTime="2020-08-28">2020.08.28</time></li>
            </ul>
          </section>

        </div>
      </main>
    </Layout>
  );
}
