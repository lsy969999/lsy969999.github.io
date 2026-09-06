export type PortfolioCollection = 'project' | 'archive';

export type PortfolioGroup = {
  id: string;
  label: string;
  position: number;
  collapsed: boolean;
};

export type PortfolioArticle = {
  id: string;
  collection: PortfolioCollection;
  project?: string;
  title: string;
  description: string;
  permalink: string;
  order: number;
  featured: boolean;
  tags: string[];
  groupPath: PortfolioGroup[];
};

export type PortfolioMetadata = {
  articles: PortfolioArticle[];
};
