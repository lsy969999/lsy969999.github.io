import {spawn} from 'node:child_process';
import {readFile, mkdir, writeFile} from 'node:fs/promises';
import {resolve, dirname} from 'node:path';
import {fileURLToPath} from 'node:url';

import {PDFDocument} from 'pdf-lib';
import {chromium} from 'playwright';

const root = dirname(fileURLToPath(import.meta.url));
const presetPath = resolve(root, process.argv[2] ?? 'portfolio-pdf.game-client-test.json');
const preset = JSON.parse(await readFile(presetPath, 'utf8'));
const outputDir = resolve(root, 'output', 'pdf');
const outputPath = resolve(outputDir, preset.outputFileName);
const port = 4173;
const origin = `http://127.0.0.1:${port}`;
const docusaurus = resolve(root, 'node_modules', '@docusaurus', 'core', 'bin', 'docusaurus.mjs');

if (!Array.isArray(preset.documents) || preset.documents.length === 0) {
  throw new Error('프리셋에 출력할 MDX documents가 없습니다.');
}

await mkdir(outputDir, {recursive: true});

const server = spawn(
  process.execPath,
  [docusaurus, 'start', '--host', '127.0.0.1', '--port', String(port), '--no-open'],
  {cwd: root, stdio: ['ignore', 'pipe', 'pipe']},
);

server.stdout.on('data', (data) => process.stdout.write(data));
server.stderr.on('data', (data) => process.stderr.write(data));

async function waitForServer() {
  for (let attempt = 0; attempt < 120; ++attempt) {
    try {
      const response = await fetch(origin);
      if (response.ok) {
        return;
      }
    } catch {
      // 개발 서버가 기동될 때까지 재시도한다.
    }
    await new Promise((resolveWait) => setTimeout(resolveWait, 500));
  }
  throw new Error('Docusaurus 개발 서버가 제한 시간 안에 시작되지 않았습니다.');
}

async function appendPdf(target, sourceBytes) {
  const source = await PDFDocument.load(sourceBytes);
  const pages = await target.copyPages(source, source.getPageIndices());
  for (const page of pages) {
    target.addPage(page);
  }
}

function escapeHtml(value) {
  return String(value)
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;');
}

async function renderCover(page) {
  const documentList = preset.documents
    .map((document) => `<li>${escapeHtml(document.title)}</li>`)
    .join('');
  await page.setContent(`
    <!doctype html>
    <html lang="ko">
      <head>
        <meta charset="utf-8" />
        <style>
          @page { size: A4; margin: 0; }
          * { box-sizing: border-box; -webkit-print-color-adjust: exact; print-color-adjust: exact; }
          body { margin: 0; color: #14213d; font-family: "Malgun Gothic", sans-serif; }
          main { display: flex; min-height: 297mm; flex-direction: column; justify-content: center; padding: 18mm 16mm; }
          .eyebrow { color: #2f6fed; font-size: 12px; font-weight: 800; letter-spacing: .14em; }
          h1 { margin: 18px 0 8px; font-size: 48px; letter-spacing: -.05em; }
          .subtitle { color: #5b6475; font-size: 21px; }
          .selection { margin-top: 42px; padding: 22px 26px; border: 1px solid #dde3ec; border-radius: 16px; background: #f3f6fa; }
          .selection strong { display: block; margin-bottom: 10px; }
          ul { margin: 0; padding-left: 20px; line-height: 1.8; }
        </style>
      </head>
      <body>
        <main>
          <p class="eyebrow">SELECTED PORTFOLIO</p>
          <h1>${escapeHtml(preset.title)}</h1>
          <p class="subtitle">${escapeHtml(preset.subtitle ?? '')}</p>
          <section class="selection">
            <strong>선택 문서</strong>
            <ul>${documentList}</ul>
          </section>
        </main>
      </body>
    </html>
  `);
  return page.pdf({format: 'A4', printBackground: true, preferCSSPageSize: true});
}

async function renderMdxDocument(page, route) {
  const response = await page.goto(`${origin}${route}`, {waitUntil: 'networkidle'});
  if (!response?.ok()) {
    throw new Error(`MDX 문서를 열지 못했습니다: ${route}`);
  }

  await page.addStyleTag({
    content: `
      @page { size: A4; margin: 16mm; }
      * { -webkit-print-color-adjust: exact; print-color-adjust: exact; }
      .navbar, .footer, .theme-doc-sidebar-container, .theme-doc-toc-desktop,
      .theme-doc-breadcrumbs, .pagination-nav, .portfolio-pagination,
      .theme-doc-footer, aside {
        display: none !important;
      }
      .main-wrapper, main, .container, .row, article, .theme-doc-markdown {
        width: 100% !important;
        max-width: none !important;
        margin: 0 !important;
        padding: 0 !important;
      }
      h1, h2, h3 { break-after: avoid; }
      pre, blockquote, table, img { break-inside: avoid; }
    `,
  });
  await page.emulateMedia({media: 'print'});
  return page.pdf({format: 'A4', printBackground: true, preferCSSPageSize: true});
}

let browser;
try {
  await waitForServer();
  browser = await chromium.launch({headless: true});
  const page = await browser.newPage({colorScheme: 'light'});
  const merged = await PDFDocument.create();

  if (preset.includeProfile) {
    await appendPdf(merged, await renderCover(page));
  }

  for (const document of preset.documents) {
    await appendPdf(merged, await renderMdxDocument(page, document.route));
  }

  await writeFile(outputPath, await merged.save());
  console.log(`PDF generated: ${outputPath}`);
} finally {
  await browser?.close();
  server.kill();
}
