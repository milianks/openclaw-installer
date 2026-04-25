import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { PageType } from '../../App';
import { RefreshCw, ExternalLink, Loader2, Sun, Moon, Sparkles } from 'lucide-react';
import { open } from '@tauri-apps/plugin-shell';
import { invoke } from '@tauri-apps/api/core';
import { useTheme } from '../../lib/ThemeContext';

interface HeaderProps {
  currentPage: PageType;
}

const pageTitles: Record<PageType, { title: string; description: string }> = {
  dashboard: { title: 'Overview', description: 'Service health, logs, and launch controls' },
  ai: { title: 'AI Models', description: 'Providers, credentials, and model routing' },
  agents: { title: 'Agents', description: 'Roles, defaults, and workspace behaviors' },
  channels: { title: 'Channels', description: 'Messaging integrations and delivery paths' },
  skills: { title: 'Skills', description: 'Capabilities, packages, and runtime tools' },
  testing: { title: 'Testing', description: 'Diagnostics, checks, and environment validation' },
  logs: { title: 'Logs', description: 'Application output and troubleshooting context' },
  security: { title: 'Security', description: 'Risk review, hardening, and safety controls' },
  settings: { title: 'Settings', description: 'Identity, language, and advanced system options' },
};

export function Header({ currentPage }: HeaderProps) {
  const { t } = useTranslation();
  const fallback = pageTitles[currentPage];
  const title = t(`header.${currentPage}.title`, { defaultValue: fallback.title });
  const description = t(`header.${currentPage}.description`, { defaultValue: fallback.description });
  const [opening, setOpening] = useState(false);
  const { theme, toggleTheme } = useTheme();

  const handleOpenDashboard = async () => {
    setOpening(true);
    try {
      const url = await invoke<string>('get_dashboard_url');
      await open(url);
    } catch (e) {
      console.error('打开 Dashboard 失败:', e);
      window.open('http://localhost:18789', '_blank');
    } finally {
      setOpening(false);
    }
  };

  return (
    <header
      className="titlebar-drag flex min-h-[88px] items-center justify-between px-7"
      style={{
        backgroundColor: 'var(--bg-overlay)',
        borderBottom: '1px solid var(--border-secondary)',
        backdropFilter: 'blur(18px)',
      }}
    >
      <div className="titlebar-no-drag min-w-0">
        <div className="mb-2 flex items-center gap-2">
          <span
            className="inline-flex items-center gap-1 rounded-full px-2.5 py-1 text-[10px] font-semibold uppercase tracking-[0.24em]"
            style={{
              color: 'var(--text-secondary)',
              backgroundColor: 'color-mix(in srgb, var(--bg-elevated) 82%, transparent)',
              border: '1px solid var(--border-secondary)',
            }}
          >
            <Sparkles size={12} />
            Workbench
          </span>
        </div>
        <h2 className="truncate text-[1.6rem] font-semibold leading-none" style={{ color: 'var(--text-primary)' }}>
          {title}
        </h2>
        <p className="mt-1 truncate text-sm" style={{ color: 'var(--text-secondary)' }}>
          {description}
        </p>
      </div>

      <div className="titlebar-no-drag flex items-center gap-3">
        <button
          onClick={toggleTheme}
          className="icon-button"
          style={{ color: 'var(--text-secondary)' }}
          title={theme === 'light' ? 'Switch to dark mode' : 'Switch to light mode'}
        >
          {theme === 'light' ? <Moon size={16} /> : <Sun size={16} />}
        </button>

        <button
          onClick={() => window.location.reload()}
          className="icon-button"
          style={{ color: 'var(--text-secondary)' }}
          title={t('header.refresh', { defaultValue: 'Refresh' })}
        >
          <RefreshCw size={16} />
        </button>

        <button
          onClick={handleOpenDashboard}
          disabled={opening}
          className="btn-secondary flex items-center gap-2 px-4 py-2.5 text-sm"
          title={t('header.openDashboard', { defaultValue: 'Open Web Dashboard' })}
        >
          {opening ? <Loader2 size={14} className="animate-spin" /> : <ExternalLink size={14} />}
          <span>Web Dashboard</span>
        </button>
      </div>
    </header>
  );
}
