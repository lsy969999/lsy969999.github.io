import {projects} from './projects';

export type TimelineEntry = {
  period: string;
  title: string;
  summary?: string;
  links?: {
    href: string;
    label: string;
  }[];
};

// 프로젝트 소개는 Projects와 같은 데이터를 사용해 제목·기간·설명의 차이를 방지한다.
function projectEntry(slug: string): TimelineEntry {
  const project = projects.find((item) => item.slug === slug);
  if (!project) {
    throw new Error(`Timeline project not found: ${slug}`);
  }

  return {
    period: project.period,
    title: project.title,
    summary: project.description,
    links: [{href: `/projects/${project.slug}`, label: project.title}],
  };
}

const timelineChronological: TimelineEntry[] = [
  {
    period: '2020.02 – 2020.08',
    title: 'KOSMO 64기 JAVA Backend 과정',
    summary:
      'Java·JSP·Spring과 데이터베이스를 배우고 웹 애플리케이션을 개발했습니다.',
  },
  {
    period: '2020.06.30',
    title: 'SQLD 취득',
  },
  projectEntry('hdbank'),
  {
    period: '2020.08.28',
    title: '정보처리산업기사 취득',
  },
  {
    period: '2020.10.12 – 2022.12.15',
    title: '(주)이*넷',
    summary:
      '사내 스타트업 부서에서 기부 플랫폼 C*erry와 C*erry Card 관리자페이지의 개발·운영을 담당했습니다. Spring Boot·PostgreSQL로 사용자·관리자 기능과 자동충전·환불 대상 계산을 구현하고, 외부 카드사 연계와 정기 배포를 맡았습니다.',
    links: [
      {href: '/projects/cherry', label: 'C*erry'},
      {href: '/projects/cherry-card', label: 'C*erry Card 관리자페이지'},
    ],
  },
  {
    period: '2021.03.01 – 2023.02.22',
    title: '한국방송통신대학교 컴퓨터과학과',
    summary: '컴퓨터과학과에 편입하여 졸업했습니다.',
  },
  {
    period: '2022.12.15 – 2024.01.01',
    title: '주식회사 C*erry',
    summary:
      '분사 이후 걷기 리워드 앱 C*erry World의 초기 개발과 출시에 참여했습니다. iOS 앱 개발을 전담하고 웹–네이티브 통신과 Android·iOS 광고 연계를 구현했습니다. 출시 후에는 앱 이전에 따른 Apple 로그인 마이그레이션과 서비스 유지보수를 맡았습니다.',
    links: [{href: '/projects/cherry-world', label: 'C*erry World'}],
  },
  projectEntry('flappy-bird'),
  projectEntry('lucky-ball'),
  projectEntry('tetris'),
  {
    period: '2025.08',
    title: '쥬신게임아카데미 교육 시작',
    summary:
      'C++ 게임 클라이언트 교육을 시작했습니다. Win32 API와 DirectX를 배우며 게임 프로그래밍의 기초를 공부했습니다.',
  },
  projectEntry('sanabi'),
  {
    period: '2025.12.24',
    title: '정보처리기사 취득',
  },
  projectEntry('dave-the-diver'),
  projectEntry('minecraft'),
  projectEntry('hogwarts-legacy'),
  {
    period: '2026.08',
    title: '쥬신게임아카데미 수료',
    summary:
      '호그와트 레거시 팀 프로젝트를 끝으로 1년간의 C++ 게임 클라이언트 교육 과정을 마쳤습니다.',
  },
];

// 최근 경력과 프로젝트가 About 페이지의 첫 화면에 먼저 보이도록 역순으로 제공한다.
export const timeline: TimelineEntry[] = [...timelineChronological].reverse();
