# Limits1214 Portfolio

Docusaurus로 작성하는 개인 포트폴리오 사이트입니다.

## Requirements

- Node.js 20 이상
- npm

## Local development

```bash
npm ci
npm run start
```

## Verification

```bash
npm run validate
npm run typecheck
npm run build
```

문서 작성 규칙과 프로젝트 등록 방법은 [CONTENT_GUIDE.md](./CONTENT_GUIDE.md)를 참고합니다.

## Deployment

`main` 브랜치에 푸시하면 GitHub Actions가 정적 사이트를 빌드해 GitHub Pages에 배포합니다.
