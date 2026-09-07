import Layout from '@theme/Layout';
import Heading from '@theme/Heading';

import CareerTimeline from '@site/src/components/CareerTimeline';
import styles from './section.module.css';

export default function About(): React.JSX.Element {
  return (
    <Layout title="About" description="임성윤의 개발 경력과 프로젝트 이력">
      <main className={styles.page}>
        <div className={styles.content}>
          <Heading as="h1" className={styles.title}>
            About
          </Heading>
          <p className={styles.description}>
            웹 서비스와 모바일 앱을 개발·운영했습니다. 지금은 게임 개발을
            공부하며, 게임플레이부터 프레임워크와 개발 도구까지 만들어 보고
            있습니다.
          </p>
          <p className={styles.description}>
            게임 전체를 이해하는 개발자가 되고 싶습니다. 맡은 기능이 다른
            시스템과 어떻게 연결되고, 실제 플레이에 어떤 영향을 주는지까지
            살피며 개발하고자 합니다. 익숙한 기술에 머무르지 않고 필요한 것을
            배우며, 함께 게임을 완성하는 데 기여하고 싶습니다.
          </p>

          <section aria-labelledby="journey-title">
            <Heading as="h2" id="journey-title" className={styles.sectionTitle}>
              이력
            </Heading>
            <CareerTimeline />
          </section>
        </div>
      </main>
    </Layout>
  );
}
