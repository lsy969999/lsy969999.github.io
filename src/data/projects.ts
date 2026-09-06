import projectDefinitions from './projects.json';

export type ProjectTask = {
  title: string;
  description: string;
};

export type PortfolioProject = {
  slug: string;
  title: string;
  period: string;
  description: string;
  role?: string;
  teamSize?: string;
  platform?: string;
  overview?: string;
  collaboration?: string[];
  tasks?: ProjectTask[];
};

export const projects: PortfolioProject[] = projectDefinitions;
