export type TimelineEntry = {
  period: string;
  title: string;
  summary: string;
  links?: {
    href: string;
    label: string;
  }[];
};

const timelineChronological: TimelineEntry[] = [
  {
    period: '2020.02 – 2020.08',
    title: 'KOSMO 64기 JAVA Backend 과정',
    summary:
      'Java, JSP, Spring과 데이터베이스를 중심으로 웹 애플리케이션 개발 과정을 수료했습니다.',
  },
  {
    period: '2020.08',
    title: 'HDBANK',
    summary:
      'KOSMO 파이널 팀 프로젝트에서 Spring·MyBatis 기반 계좌이체 기능과 JSP 화면을 구현했습니다.',
    links: [{href: '/projects/hdbank', label: 'HDBANK'}],
  },
  {
    period: '2020.10.12 – 2022.12.15',
    title: '(주)이*넷',
    summary:
      '기부 플랫폼 스타트업 부서인 C*erry에서 C*erry 플랫폼과 C*erry Card 서비스를 개발했습니다. Spring Boot와 PostgreSQL을 사용했으며 Kubernetes, Grafana, Argo 기반 운영 환경을 경험했습니다.',
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
      '(주)이*넷의 스타트업 부서에서 분사한 주식회사 C*erry에서 C*erry World 앱을 신규 출시하고 서비스를 유지보수했습니다. iOS 개발을 전담하고 Android·iOS 광고 연동을 담당했습니다.',
    links: [{href: '/projects/cherry-world', label: 'C*erry World'}],
  },
  {
    period: '2024.08',
    title: 'Flappy Bird',
    summary:
      'Rust와 Bevy 엔진으로 게임을 구현하고 Android, iOS, Web 환경에서 사용할 수 있도록 FFI 연결을 다뤘습니다.',
    links: [{href: '/projects/flappy-bird', label: 'Flappy Bird'}],
  },
  {
    period: '2024.10 – 2025.01',
    title: 'Lucky Ball',
    summary:
      'Rust와 Bevy 엔진으로 게임을 구현하고 Android, iOS, Web에서 공유하기 위한 FFI 구조와 플랫폼 연동을 구현했습니다.',
    links: [{href: '/projects/lucky-ball', label: 'Lucky Ball'}],
  },
  {
    period: '2025.06 – 2025.08',
    title: 'Tetris',
    summary:
      'Rust 게임 코어를 Android, iOS, Web과 FFI로 연결하고, WebSocket을 이용한 웹 멀티플레이 게임을 구현했습니다.',
    links: [{href: '/projects/tetris', label: 'Tetris'}],
  },
  {
    period: '2025.08',
    title: '쥬신게임아카데미 교육 시작',
    summary:
      'C++ 게임 클라이언트 교육을 시작하고, 프레임워크부터 렌더링, 물리와 도구 개발까지 단계적으로 프로젝트를 수행했습니다.',
  },
  {
    period: '2025.11 – 2025.12',
    title: 'SANABI',
    summary:
      'Win32 API와 GDI 기반 프레임워크를 직접 구성하고 게임플레이 시스템을 구현했습니다.',
    links: [{href: '/projects/sanabi', label: 'SANABI'}],
  },
  {
    period: '2026.01 – 2026.03',
    title: 'DAVE THE DIVER',
    summary:
      'DirectX 9 기반 팀 프로젝트에서 팀장을 맡아 개발을 이끌고 게임 클라이언트 기능을 구현했습니다.',
    links: [{href: '/projects/dave-the-diver', label: 'DAVE THE DIVER'}],
  },
  {
    period: '2026.04 – 2026.06',
    title: 'Minecraft',
    summary:
      'DirectX 11 기반 복셀 렌더링과 커스텀 엔진 구조를 구현하며 그래픽스 역량을 확장했습니다.',
    links: [{href: '/projects/minecraft', label: 'Minecraft'}],
  },
  {
    period: '2026.06 – 2026.08',
    title: 'Hogwarts Legacy',
    summary:
      '팀장과 프레임워크 담당으로 엔진, 물리 시뮬레이션, 렌더링, 도구와 게임플레이를 아우르는 작업을 수행했습니다.',
    links: [{href: '/projects/hogwarts-legacy', label: 'Hogwarts Legacy'}],
  },
  {
    period: '2026.08',
    title: '쥬신게임아카데미 수료',
    summary:
      'C++ 게임 클라이언트 교육 과정과 최종 팀 프로젝트를 마치고 수료했습니다.',
  },
  {
    period: '현재',
    title: '게임 클라이언트 프로그래머 지원 준비',
    summary:
      '프로젝트의 결과뿐 아니라 문제 정의, 설계 판단, 디버깅과 개선 과정을 포트폴리오로 정리하고 있습니다.',
  },
];

// 최근 경력과 프로젝트가 About 페이지의 첫 화면에 먼저 보이도록 역순으로 제공한다.
export const timeline: TimelineEntry[] = [...timelineChronological].reverse();
