import OriginalCodeBlock from '@theme-original/CodeBlock';
import type {Props} from '@theme/CodeBlock';

const LANGUAGE_LABELS: Record<string, string> = {
  bash: 'Bash',
  c: 'C',
  cpp: 'C++',
  cs: 'C#',
  csharp: 'C#',
  css: 'CSS',
  glsl: 'GLSL',
  hlsl: 'HLSL',
  html: 'HTML',
  java: 'Java',
  javascript: 'JavaScript',
  js: 'JavaScript',
  json: 'JSON',
  jsx: 'JSX',
  lua: 'Lua',
  markdown: 'Markdown',
  md: 'Markdown',
  mdx: 'MDX',
  plaintext: 'Plain Text',
  powershell: 'PowerShell',
  ps1: 'PowerShell',
  shell: 'Shell',
  sql: 'SQL',
  text: 'Plain Text',
  ts: 'TypeScript',
  typescript: 'TypeScript',
  tsx: 'TSX',
  xml: 'XML',
  yaml: 'YAML',
  yml: 'YAML',
};

function getLanguage({className, language}: Props): string {
  if (typeof language === 'string' && language.length > 0) {
    return language.toLowerCase();
  }

  if (typeof className === 'string') {
    const match = className.match(/(?:^|\s)language-([^\s]+)/);
    if (match?.[1]) {
      return match[1].toLowerCase();
    }
  }

  return 'plaintext';
}

export default function CodeBlock(props: Props): React.JSX.Element {
  const language = getLanguage(props);
  const languageLabel = LANGUAGE_LABELS[language] ?? language.toUpperCase();
  const explicitTitle =
    typeof props.title === 'string' && props.title.trim().length > 0
      ? props.title.trim()
      : undefined;

  return (
    <OriginalCodeBlock
      {...props}
      title={explicitTitle ? `${explicitTitle} · ${languageLabel}` : languageLabel}
    />
  );
}
