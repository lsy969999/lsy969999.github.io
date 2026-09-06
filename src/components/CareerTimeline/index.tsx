import Link from '@docusaurus/Link';
import Heading from '@theme/Heading';

import {timeline} from '@site/src/data/timeline';
import styles from './styles.module.css';

export default function CareerTimeline(): React.JSX.Element {
  return (
    <ol className={styles.timeline} aria-label="개발 경력 타임라인">
      {timeline.map((entry, index) => (
        <li className={styles.item} key={`${entry.period}-${entry.title}`}>
          <p className={styles.period}>{entry.period}</p>
          <div className={styles.rail} aria-hidden="true">
            <span className={styles.marker}>{index + 1}</span>
          </div>
          <article className={styles.card}>
            <Heading as="h2" className={styles.title}>
              {entry.title}
            </Heading>
            <p className={styles.summary}>{entry.summary}</p>
            {entry.links && entry.links.length > 0 && (
              <div className={styles.links} aria-label={`${entry.title} 관련 프로젝트`}>
                {entry.links.map((link) => (
                  <Link className={styles.link} key={link.href} to={link.href}>
                    {link.label} →
                  </Link>
                ))}
              </div>
            )}
          </article>
        </li>
      ))}
    </ol>
  );
}
