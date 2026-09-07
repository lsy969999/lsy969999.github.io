import {spawn} from 'node:child_process';
import {readFile, mkdir, writeFile} from 'node:fs/promises';
import {resolve, dirname, basename} from 'node:path';
import {fileURLToPath} from 'node:url';
import {createServer} from 'node:net';
import {PDFDocument, StandardFonts, rgb} from 'pdf-lib';
import {chromium} from 'playwright';

const root = dirname(fileURLToPath(import.meta.url));
const preset = JSON.parse(await readFile(resolve(root, process.argv[2] ?? 'portfolio-pdf.game-client-test.json'), 'utf8'));
const projects = JSON.parse(await readFile(resolve(root, 'src/data/projects.json'), 'utf8'));
if (!Array.isArray(preset.documents) || preset.documents.length === 0) {
  throw new Error('프리셋에 출력할 MDX documents가 없습니다.');
}
if (basename(preset.outputFileName) !== preset.outputFileName || !preset.outputFileName.endsWith('.pdf')) {
  throw new Error('outputFileName에는 경로 없이 PDF 파일명만 입력하세요.');
}
const outputDir = resolve(root, 'output/pdf');
const outputPath = resolve(outputDir, preset.outputFileName);
const styles = await readFile(resolve(root, 'portfolio-pdf.css'), 'utf8');
await mkdir(outputDir, {recursive: true});

// 실행 중인 사용자 개발 서버와 겹치지 않는 포트를 사용한다.
const port = await new Promise((resolvePort, reject) => {
  const probe = createServer();
  probe.once('error', reject);
  probe.listen(0, '127.0.0.1', () => {
    const value = probe.address().port;
    probe.close(() => resolvePort(value));
  });
});
const origin = `http://127.0.0.1:${port}`;
const publicOrigin = preset.siteUrl ?? 'https://limits1214.github.io';
const docusaurus = resolve(root, 'node_modules/@docusaurus/core/bin/docusaurus.mjs');
const server = spawn(process.execPath,
  [docusaurus, 'start', '--host', '127.0.0.1', '--port', String(port), '--no-open'],
  {cwd: root, stdio: ['ignore', 'pipe', 'pipe']});
let serverError;
server.on('error', error => { serverError = error; });
server.stdout.on('data', data => process.stdout.write(data));
server.stderr.on('data', data => process.stderr.write(data));

async function waitForServer() {
  for (let attempt = 0; attempt < 240; ++attempt) {
    if (serverError) throw serverError;
    if (server.exitCode !== null) throw new Error(`개발 서버 종료: ${server.exitCode}`);
    try {
      const response = await fetch(origin, {signal: AbortSignal.timeout(2000)});
      if (response.ok) return;
    } catch { /* 서버 시작 대기 */ }
    await new Promise(done => setTimeout(done, 500));
  }
  throw new Error('Docusaurus 개발 서버 시작 시간이 초과됐습니다.');
}

function escapeHtml(value) {
  return String(value ?? '').replaceAll('&', '&amp;').replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;').replaceAll('"', '&quot;');
}

async function printHtml(page, body, cover = false) {
  await page.setContent(`<!doctype html><html lang="ko"><head><meta charset="utf-8">
    <style>${styles}</style></head><body class="${cover ? 'cover' : 'document'} ${cover && preset.compactProfile ? 'compact-profile' : ''}">${body}</body></html>`,
  {waitUntil: 'networkidle'});
  await page.evaluate(async () => {
    await document.fonts.ready;
    await Promise.all([...document.images].map(img => img.decode()));
  });
  return page.pdf({format: 'A4', printBackground: true, preferCSSPageSize: true});
}

async function renderMdxDocument(page, entry, index) {
  if (!/^\/(projects|archive)\//.test(entry.route ?? '')) throw new Error(`문서 경로가 아닙니다: ${entry.route}`);
  const response = await page.goto(`${origin}${entry.route}`, {waitUntil: 'networkidle'});
  if (!response?.ok()) throw new Error(`MDX 문서를 열지 못했습니다: ${entry.route}`);
  await page.waitForSelector('.theme-doc-markdown');

  // 렌더된 MDX에서 지정한 H2와 그 본문을 추출한다. 원문 파일은 변경하지 않는다.
  const content = await page.evaluate(({sections, omitParagraphsStartingWith, publicOrigin}) => {
    const source = document.querySelector('.theme-doc-markdown').cloneNode(true);
    source.querySelectorAll('button, .hash-link, .portfolio-pagination, script, style').forEach(node => node.remove());
    const headings = [...source.querySelectorAll('h2')];
    const matched = new Set();
    if (sections) {
      for (const name of sections) {
        const candidates = headings.filter(h => h.id === name || h.textContent.trim() === name);
        if (candidates.length !== 1) throw new Error(`H2 선택 실패 (${candidates.length}개): ${name}`);
        matched.add(candidates[0]);
      }
    }
    const output = document.createElement('div');
    let include = !sections;
    for (const child of [...source.children]) {
      if (child.tagName === 'H1' || child.tagName === 'HEADER') continue;
      if (child.tagName === 'H2') include = !sections || matched.has(child);
      if (include) output.append(child.cloneNode(true));
    }
    // 웹의 다른 글 안내처럼 PDF에서 생략할 문단은 프리셋에 명시한다.
    for (const prefix of omitParagraphsStartingWith ?? []) {
      const matches = [...output.querySelectorAll('p')].filter(p => p.textContent.trim().startsWith(prefix));
      if (matches.length !== 1) throw new Error(`생략 문단 선택 실패: ${prefix}`);
      matches[0].remove();
    }
    output.querySelectorAll('pre').forEach(pre => {
      const clean = document.createElement('pre');
      const code = document.createElement('code');
      const lines = [...pre.querySelectorAll('.token-line')];
      // Prism은 줄바꿈 대신 block span을 사용하므로 줄 단위로 복원한다.
      if (lines.length > 0) {
        code.textContent = lines.map(line => line.textContent).join('\n');
      } else {
        code.textContent = pre.querySelector('code')?.textContent ?? pre.textContent;
      }
      clean.append(code);
      const wrapper = pre.closest('[class*="codeBlockContainer"]');
      (wrapper ?? pre).replaceWith(clean);
    });
    output.querySelectorAll('a[href]').forEach(link => {
      const url = new URL(link.getAttribute('href'), location.href);
      if (url.origin === location.origin) {
        link.href = `${publicOrigin}${url.pathname}${url.search}${url.hash}`;
      }
    });
    output.querySelectorAll('img').forEach(img => { img.src = new URL(img.getAttribute('src'), location.href).href; });
    output.querySelectorAll('*').forEach(node => {
      node.removeAttribute('class');
      node.removeAttribute('style');
    });
    return output.innerHTML;
  }, {sections: entry.sections, omitParagraphsStartingWith: entry.omitParagraphsStartingWith, publicOrigin});

  const project = projects.find(item => item.slug === entry.project);
  if (entry.project && !project) throw new Error(`프로젝트를 찾을 수 없습니다: ${entry.project}`);
  const metadata = project ? `${project.period} · ${project.teamSize} · ${project.role}` : '';
  return printHtml(page, `<header class="chapter-heading">
    <div class="chapter-label">${String(index + 1).padStart(2, '0')} / ${escapeHtml(project?.title ?? preset.subtitle)}</div>
    <h1>${escapeHtml(entry.title)}</h1>
    <p class="metadata">${escapeHtml(metadata)}</p>
    <a class="source-link" href="${escapeHtml(publicOrigin + entry.route)}">전체 문서 ↗</a>
    </header><article>${content}</article>`);
}

async function renderCover(page, entries) {
  const selectedProjects = [...new Set(preset.documents.map(entry => entry.project).filter(Boolean))]
    .map(slug => projects.find(project => project.slug === slug));
  const projectCards = selectedProjects.map(project => `<section class="project-card">
    <h2>${escapeHtml(project.title)}</h2>
    <div class="metadata">${escapeHtml(project.period)} · ${escapeHtml(project.teamSize)} · ${escapeHtml(project.platform)}</div>
    <p>${escapeHtml(project.description)}</p>
    ${project.githubUrl ? `<a class="repository-link" href="${escapeHtml(project.githubUrl)}">GitHub 저장소 ↗</a>` : ''}
  </section>`).join('');
  const contents = entries.map(entry => `<li><span>${escapeHtml(entry.title)}</span><b>${entry.page}</b></li>`).join('');
  return printHtml(page, `<main>
    <div class="cover-rule"></div><p class="cover-category">${escapeHtml(preset.coverLabel ?? '게임 클라이언트 포트폴리오')}</p>
    <h1>${escapeHtml(preset.author ?? preset.title)}</h1>
    <p class="aspiration">${escapeHtml(preset.introduction ?? preset.subtitle)}</p>
    <div class="links"><a href="${escapeHtml(publicOrigin)}">limits1214.github.io</a>
      <span> / </span><a href="https://github.com/limits1214">GitHub · limits1214</a></div>
    <div class="projects">${projectCards}</div>
    <section class="contents"><h2>핵심 구현</h2><ol>${contents}</ol></section>
  </main>`, true);
}

async function appendPdf(target, source) {
  const pages = await target.copyPages(source, source.getPageIndices());
  pages.forEach(page => target.addPage(page));
}

let browser;
try {
  await waitForServer();
  browser = await chromium.launch({headless: true});
  const page = await browser.newPage({colorScheme: 'light'});
  await page.emulateMedia({media: 'print'});
  const documents = [];
  let nextPage = preset.includeProfile ? 2 : 1;
  for (const [index, entry] of preset.documents.entries()) {
    const pdf = await PDFDocument.load(await renderMdxDocument(page, entry, index));
    documents.push({title: entry.title, pdf, page: nextPage});
    console.log(`${entry.title}: ${pdf.getPageCount()} pages (from ${nextPage})`);
    nextPage += pdf.getPageCount();
  }
  const merged = await PDFDocument.create();
  if (preset.includeProfile) {
    const cover = await PDFDocument.load(await renderCover(page, documents));
    if (cover.getPageCount() !== 1) throw new Error('표지가 한 페이지를 초과했습니다. 표지 내용을 줄여 주세요.');
    await appendPdf(merged, cover);
  }
  for (const entry of documents) await appendPdf(merged, entry.pdf);
  const font = await merged.embedFont(StandardFonts.Helvetica);
  const count = merged.getPageCount();
  merged.getPages().forEach((page, index) => {
    const {width} = page.getSize();
    page.drawLine({start: {x: 48, y: 33}, end: {x: width - 48, y: 33}, thickness: 0.4, color: rgb(.83, .86, .89)});
    page.drawText('limits1214 | Game Client Portfolio', {x: 48, y: 21, size: 8, font, color: rgb(.38, .44, .49)});
    const number = `${index + 1} / ${count}`;
    page.drawText(number, {x: width - 48 - font.widthOfTextAtSize(number, 8), y: 21, size: 8, font, color: rgb(.38, .44, .49)});
  });
  merged.setTitle(preset.title);
  merged.setAuthor(preset.author ?? '임성윤');
  merged.setSubject(preset.subtitle ?? '게임 클라이언트 핵심 구현');
  await writeFile(outputPath, await merged.save());
  console.log(`PDF generated (${count} pages): ${outputPath}`);
} finally {
  await browser?.close();
  server.kill();
}
