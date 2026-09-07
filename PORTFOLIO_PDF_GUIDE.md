# Local portfolio PDF export

Docusaurus가 `projects` 폴더의 MDX를 웹 문서로 생성하고, Playwright Chromium이 프리셋에 선택된 문서 URL을 직접 출력한다. 여러 문서는 하나의 PDF로 병합하며 결과는 로컬 `output/pdf`에만 생성한다.

## 실행

네 게임의 대표 구현을 모은 기술서:

```bash
npm run pdf:projects
```

`portfolio-pdf.game-projects.json`에서 호그와트 레거시·마인크래프트·데버다·산나비의 대표 구현 6개를 선택한다. 결과는 `output/pdf/ImSeongYun_Game_Portfolio_Technical.pdf`이며, 기존 2개 프로젝트 PDF와 별도로 저장한다.

`compactProfile`은 네 프로젝트 소개를 표지의 2열 카드로 배치한다. `coverLabel`은 표지 상단 문구다. 각 문서의 `omitParagraphsStartingWith`에는 PDF에서 생략할 문단의 시작 문장을 지정할 수 있다. MDX의 강조·코드 표시는 제외한 화면상의 문자열을 사용하며, 일치하는 문단이 하나가 아니면 출력이 중단된다. 웹의 다른 글 안내나 사용하지 않는 테스트 경로 설명을 빼는 데 사용한다.

핵심 문서의 선택한 절만 모은 게임 클라이언트 포트폴리오:

```bash
npm run pdf:core
```

`portfolio-pdf.game-client-core.json`을 사용하며 `output/pdf/ImSeongYun_GameClient_Portfolio_Core.pdf`에 저장한다. 웹 경력은 포함하지 않는다.

기존 단일 문서 테스트:

```bash
npm run pdf:test
```

출력 파일:

```text
output/pdf/GameClient_Portfolio_Test.pdf
```

`output` 폴더는 `.gitignore`에 포함되므로 생성된 제출용 PDF가 저장소에 올라가지 않는다.

## 프리셋

`portfolio-pdf.game-client-test.json`의 `documents`에 출력할 MDX 문서 경로를 순서대로 작성한다.

- `includeProfile`: 로컬 PDF 앞에 표지를 추가한다.
- `documents`: 출력할 MDX의 제목과 웹 경로 목록이다.
- `outputFileName`: 로컬에 생성될 파일명이다.

핵심 프리셋은 문서마다 다음과 같이 절을 선택한다.

```json
{
  "project": "hogwarts-legacy",
  "title": "Generation Handle과 객체 수명 관리",
  "route": "/projects/hogwarts-legacy/handle-object-manager",
  "sections": ["배경과 목표", "Handle 구조", "즉시 삭제 대신 FrameEnd 지연 파괴", "구현 결과"]
}
```

- `project`: `src/data/projects.json`의 slug. 기간·팀 인원·역할은 기존 프로젝트 데이터에서 읽는다.
- 프로젝트 데이터에 `githubUrl`을 지정하면 웹 프로젝트 개요와 PDF 표지에 저장소 링크가 표시된다. 주소가 없는 프로젝트에는 링크를 만들지 않는다.
- `title`: PDF에서 표시할 장 제목. 웹 문서 제목을 변경하지 않는다.
- `sections`: 포함할 H2 제목 또는 heading ID. 본문은 원문의 순서로 출력하며, H3 이하도 함께 포함한다. 생략하면 문서 전체를 출력한다.
- 제목이 중복되면 heading ID로 지정한다. 제목을 바꿨거나 선택한 절이 없으면 조용히 누락하지 않고 오류로 중단한다.
- `author`, `introduction`: 표지에 들어갈 이름과 소개 문구.

표지 목차의 페이지 번호와 전체 페이지 번호는 자동 계산한다. 웹 내비게이션·태그·이전/다음 문서 버튼은 제외하고, 문서 링크는 공개 사이트 주소로 바꾼다. PDF만의 여백·글자 크기·색상은 `portfolio-pdf.css`에서 조절한다. 실행 중인 개발 서버와 별도 포트로 임시 서버를 열고 출력 후 종료한다.

기술 문서의 원본은 `projects` 아래의 MDX다. TSX 래퍼나 별도의 PDF 본문은 사용하지 않는다. MDX를 수정하면 웹과 다음 PDF 출력에 동일하게 반영된다.

## 프로젝트 문서 자동 등록

새 MDX는 front matter의 `project`가 가리키는 프로젝트 상세 페이지에 자동으로 표시된다. `projects.ts`에 문서 정보를 다시 적지 않는다.

```mdx
---
id: physx-ragdoll
project: hogwarts-legacy
title: PhysX 랙돌 시스템
description: 플레이어 사망 상태에 랙돌을 통합한 과정
slug: /hogwarts-legacy/physx-ragdoll
order: 20
featured: false
tags:
  - PhysX
  - Ragdoll
---
```

- `project`: 문서를 표시할 프로젝트의 slug다.
- `order`: 프로젝트 상세의 기술 문서 정렬 순서다.
- `featured`: 이후 대표 문서 필터에 사용할 값이다.
- `tags`: 문서 카드와 Docusaurus 태그 페이지에서 사용한다.

필수 문자열인 `id`, `project`, `title`, `description`, `slug`가 빠지면 빌드가 실패하므로 등록 누락을 바로 확인할 수 있다.

## 독립 Archive 문서

특정 프로젝트에 속하지 않는 기술 기록은 `archive` 폴더에 작성한다. Archive 문서는 `project` 필드가 필요하지 않으며 `/archive` 목록에만 자동 등록된다.

```mdx
---
id: stable-handle-lifetime
title: C++ 객체 수명과 안정적인 Handle 관리
description: 세대 기반 Handle로 객체 수명을 검증하는 방식을 정리합니다.
slug: /cpp/stable-handle-lifetime
order: 10
featured: false
tags:
  - C++
  - Handle
---
```

- `projects` 문서: `/projects/...`에 생성되며 소속 프로젝트 상세에만 표시된다.
- `archive` 문서: `/archive/...`에 생성되며 Archive에만 표시된다.
- Archive 문서도 PDF 프리셋의 `documents`에 경로를 명시하면 제출용 PDF에 포함할 수 있다.

## N-depth 문서 그룹

문서 그룹은 front matter 문자열이 아니라 실제 폴더 계층으로 관리한다. 폴더마다 `_category_.json`을 두면 화면 표시명, 순서와 기본 접힘 상태를 지정할 수 있다.

```text
projects/hogwarts-legacy/
└─ physics-simulation/
   ├─ _category_.json
   └─ cloth/
      ├─ _category_.json
      └─ nvcloth-cape.mdx
```

```json
{
  "label": "Physics & Simulation",
  "position": 20,
  "collapsed": false
}
```

- 폴더는 필요한 만큼 중첩할 수 있다.
- `label`: 화면에 표시할 그룹 이름이다. 생략하면 폴더명을 보기 좋게 변환한다.
- `position`: 같은 깊이의 그룹 정렬 순서다. 생략하면 999다.
- `collapsed`: `true`이면 처음 화면에서 해당 그룹을 접어 둔다.
- MDX의 `order`: 같은 그룹 안에서 문서가 표시되는 순서다.
- MDX의 `slug`: 파일을 다른 그룹으로 옮겨도 기존 URL을 유지하고 싶을 때 그대로 둔다.

## Playwright 브라우저 설치

처음 한 번 Chromium 설치가 필요하다.

```bash
npx playwright install chromium
```
