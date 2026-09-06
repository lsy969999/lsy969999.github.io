# Local portfolio PDF export

Docusaurus가 `projects` 폴더의 MDX를 웹 문서로 생성하고, Playwright Chromium이 프리셋에 선택된 문서 URL을 직접 출력한다. 여러 문서는 하나의 PDF로 병합하며 결과는 로컬 `output/pdf`에만 생성한다.

## 실행

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
