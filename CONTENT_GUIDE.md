# Portfolio content guide

프로젝트와 기술 아카이브의 원본은 MDX로 작성합니다. 일반 설명은 Markdown으로 쓰고, 복잡한 표현이나 상호작용이 필요한 부분만 React 컴포넌트를 사용합니다.

## 프로젝트 추가

`src/data/projects.json`에 프로젝트 정보를 한 번만 추가합니다. `/projects/{slug}` 상세 페이지는 자동으로 생성되므로 별도의 TSX 페이지를 만들지 않습니다.

```json
{
  "slug": "new-project",
  "title": "New Project",
  "period": "2026.01 – 2026.03",
  "description": "프로젝트를 한 문장으로 설명합니다."
}
```

### 프로젝트 소개 구성

소개 화면은 **프로젝트 개요 → 담당 작업 → 협업 → 기술 문서** 순서입니다. 값이 없는 선택 항목은 표시하지 않습니다.

- `overview`: 프로젝트의 목적·구성과 특징. 본인의 구현 목록은 아래 `tasks`에 작성합니다.
- `tasks`: `title`(작업명)과 `description`(구현 방식·개선 내용)으로 구성한 배열. 기존 담당 범위와 주요 기여는 이 필드 하나로 합쳤습니다.
- `collaboration`: 다른 담당자와 연결하거나 조율한 내용의 문자열 배열. 개인 프로젝트는 생략할 수 있습니다.
- `role`, `teamSize`, `platform`: 역할, 참여 인원, 플랫폼. `period`는 기간입니다.

```json
{
  "overview": "프로젝트의 목적과 주요 구성을 설명합니다.",
  "tasks": [
    {
      "title": "객체 생명주기 관리",
      "description": "세대 기반 핸들과 지연 파괴로 외부 참조의 유효성을 검증했습니다."
    }
  ],
  "collaboration": ["플레이어 담당자가 사용할 공용 API와 호출 시점을 조율했습니다."]
}
```

`responsibilities`와 `highlights`에 같은 작업을 나누어 등록하지 않습니다. 기본 소개는 JSON, 상세 기술 설명은 아래 MDX 문서에 작성합니다.

## 프로젝트 문서

`projects/{project}/` 아래에 `.mdx` 파일을 작성합니다. 폴더는 필요한 만큼 중첩할 수 있습니다.

```mdx
---
id: physics-ragdoll
project: hogwarts-legacy
title: PhysX 랙돌 시스템
description: 플레이어 사망 상태에 랙돌을 통합한 과정
slug: /hogwarts-legacy/physics-ragdoll
order: 20
featured: false
tags:
  - PhysX
  - Ragdoll
---

# PhysX 랙돌 시스템

문제, 판단, 구현, 검증 결과 순서로 작성합니다.
```

### 문서 공통 양식

문서의 주제에 맞는 설명 순서는 유지하되, 같은 역할의 절은 아래 명칭을 사용합니다. 없는 근거나 결과를 채우기 위해 빈 절을 만들지 않습니다.

| 역할 | 공통 제목 |
| --- | --- |
| 제목 아래의 도입 설명 | 개요 |
| 해결하려던 문제·설계 목적 | 배경과 목표 |
| 본인이 맡은 범위를 함께 설명 | 목표와 담당 범위 |
| 실제 구현 결과 | 구현 결과 |
| 결과와 제약을 함께 설명 | 결과와 한계 |
| 확인된 결과·기여 범위를 함께 설명 | 결과와 확인 범위 |
| 설계 선택의 비용·후속 보강 | 한계와 개선 방향 |
| 원문·코드·커밋과 확인 수준 | 작성 근거와 확인 범위 |
| 근거와 아직 수행하지 않은 시연 | 작성 근거와 검증 계획 |
| 앞으로 실행할 테스트·시연 | 검증 계획 |
| 다른 글로 연결 | 관련 문서 / 추천 문서 |

- H1은 문서당 하나이며 front matter의 `title`과 일치시킵니다. H2는 주요 절, H3는 그 절의 세부 항목입니다.
- H2에 `1.`, `2.` 같은 장 번호를 붙이지 않습니다. 실제 절차·알고리즘 단계에는 번호 목록이나 H3 번호를 사용할 수 있습니다.
- 구현 설명의 구체적인 절 제목은 자유롭게 작성합니다. 모든 글을 동일한 절 개수로 억지로 맞추지 않습니다.
- 기존 제목을 바꿀 때는 `{/* #기존-id */}`로 앵커를 유지해 외부 링크와 문서 내 링크가 끊기지 않게 합니다. 이번 통일 작업에서도 기존 앵커를 유지했습니다.
- 본문은 `구현했다`, `확인했다` 형태의 서술체를 사용합니다. 인용문·코드·실제 UI 명칭은 원문을 유지합니다.
- 코드블록에는 `cpp`, `rust`, `sql`, `text` 등 언어를 지정합니다. 설명용 재구성 코드와 실제 코드 발췌는 본문 또는 코드 제목에서 구분합니다.
- 로컬 캡처는 이미지를 같은 파일로 링크해 클릭하면 크게 볼 수 있게 합니다. 캡션의 짧은 설명은 이탤릭으로, 해석·마스킹·비교 시 주의점은 그 뒤 일반 문장으로 적습니다.
- 실제 측정·실행 결과와 소스 검토, 담당자 회고, 앞으로의 검증 계획을 구분합니다. 양식 통일을 위해 수행하지 않은 테스트나 근거를 추가하지 않습니다.
- 원본 로그·이미지의 민감 정보는 마스킹한 사본만 첨부합니다.

```mdx
# 문서 제목

## 개요

무엇을 구현했는지 간단히 설명한다.

## 배경과 목표

해결하려던 문제를 설명한다.

## 구체적인 설계·구현 주제

선택한 방식과 동작 흐름을 설명한다.

## 결과와 한계

확인한 결과와 남은 제약을 구분한다.

## 작성 근거와 확인 범위

소스·커밋·측정 자료와 실제 확인한 수준을 적는다.
```

## Archive 문서

특정 프로젝트에 종속되지 않는 글은 `archive/` 아래에 작성합니다. 프로젝트 문서와 달리 `project` 필드가 필요하지 않습니다.

```mdx
---
id: stable-handle-lifetime
title: 안정적인 Handle 관리
description: 객체 수명을 세대 기반 Handle로 검증한 방법
slug: /cpp/stable-handle-lifetime
order: 10
featured: false
tags:
  - C++
  - Handle
---
```

## 작성 중인 문서

작성 중인 문서는 `draft: true`를 지정합니다. 로컬 개발 서버에서는 확인할 수 있지만 프로덕션 빌드와 배포 목록에서는 제외됩니다.

```mdx
---
id: work-in-progress
project: hogwarts-legacy
title: 작성 중인 문서
description: 아직 작성 중인 문서입니다.
slug: /hogwarts-legacy/work-in-progress
order: 999
featured: false
tags: []
draft: true
---
```

공개 저장소에 커밋하면 MDX 원본 자체는 GitHub에서 볼 수 있습니다. 외부에 공개하면 안 되는 내용은 비공개 저장소나 로컬에서 관리합니다.

## 문서 그룹

폴더의 `_category_.json`으로 트리의 이름과 순서를 지정합니다.

```json
{
  "label": "Physics & Simulation",
  "position": 20,
  "collapsed": false
}
```

## 코드블럭

백틱 뒤에 언어를 지정하면 구문 강조와 언어 이름이 함께 표시됩니다.

````md
```cpp
auto handle = CreateHandle();
```
````

파일명을 명시하면 `파일명 · C++`처럼 표시됩니다.

````md
```cpp title="GameObject.cpp"
void CGameObject::Update() {}
```
````

## React 컴포넌트 사용

재사용할 UI는 `src/components/`에 작성하고 MDX에서 가져옵니다.

```mdx
import PerformanceChart from '@site/src/components/PerformanceChart';

<PerformanceChart />
```

HTML 형태의 태그도 JSX 규칙을 따릅니다. `class` 대신 `className`을 사용하고 `<img />`처럼 태그를 닫아야 합니다.

## 검사

커밋 전 아래 명령을 실행합니다.

```bash
npm run validate
npm run typecheck
npm run build
```

`validate`는 프로젝트 데이터, 문서의 필수 값, 존재하지 않는 프로젝트, 중복된 문서 ID와 URL, `_category_.json` 형식을 검사합니다.

`projects` 또는 `archive`에 첫 MDX 문서를 추가한 경우 개발 서버를 한 번 다시 시작합니다. 문서가 없는 컬렉션은 빌드 오류를 막기 위해 자동으로 비활성화되며, 첫 문서가 생기면 다시 활성화됩니다.
