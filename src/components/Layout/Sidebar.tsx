import { motion } from 'framer-motion';
import {
  LayoutDashboard,
  Bot,
  Users,
  MessageSquare,
  Puzzle,
  FlaskConical,
  ScrollText,
  Settings,
  ShieldAlert,
} from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { PageType } from '../../App';
import { BrandMark } from '../BrandMark';
import clsx from 'clsx';

interface ServiceStatus {
  running: boolean;
  pid: number | null;
  port: number;
}

interface SidebarProps {
  currentPage: PageType;
  onNavigate: (page: PageType) => void;
  serviceStatus: ServiceStatus | null;
}

const menuItems: { id: PageType; label: string; icon: React.ElementType }[] = [
  { id: 'dashboard', label: 'Overview', icon: LayoutDashboard },
  { id: 'ai', label: 'AI Config', icon: Bot },
  { id: 'agents', label: 'Agents', icon: Users },
  { id: 'channels', label: 'Channels', icon: MessageSquare },
  { id: 'skills', label: 'Skills', icon: Puzzle },
  { id: 'testing', label: 'Testing', icon: FlaskConical },
  { id: 'logs', label: 'Logs', icon: ScrollText },
  { id: 'security', label: 'Security', icon: ShieldAlert },
  { id: 'settings', label: 'Settings', icon: Settings },
];

export function Sidebar({ currentPage, onNavigate, serviceStatus }: SidebarProps) {
  const { t } = useTranslation();
  const isRunning = serviceStatus?.running ?? false;

  return (
    <aside
      className="relative flex h-full w-[304px] shrink-0 flex-col overflow-hidden rounded-[30px] border"
      style={{
        backgroundColor: 'var(--bg-sidebar)',
        borderColor: 'var(--border-primary)',
        boxShadow: 'var(--shadow-card)',
      }}
    >
      <div className="pointer-events-none absolute inset-0 bg-gradient-radial opacity-80" />

      <div
        className="relative px-6 pt-6 pb-5 titlebar-drag"
        style={{ borderBottom: '1px solid var(--border-secondary)' }}
      >
        <div className="titlebar-no-drag flex items-center gap-4">
          <div
            className="flex h-12 w-12 items-center justify-center rounded-[18px] border"
            style={{
              background: 'linear-gradient(145deg, rgba(217, 121, 79, 0.2), rgba(111, 125, 217, 0.14))',
              borderColor: 'rgba(201, 92, 52, 0.16)',
            }}
          >
            <BrandMark className="h-8 w-8" />
          </div>
          <div className="min-w-0">
            <p
              className="text-[10px] font-semibold uppercase tracking-[0.28em]"
              style={{ color: 'var(--text-tertiary)' }}
            >
              OpenClaw
            </p>
            <h1 className="text-[1.35rem] font-semibold leading-none" style={{ color: 'var(--text-primary)' }}>
              Manager
            </h1>
            <p className="mt-1 text-xs" style={{ color: 'var(--text-secondary)' }}>
              Control Room
            </p>
          </div>
        </div>
      </div>

      <nav className="relative min-h-0 flex-1 overflow-y-auto px-4 py-5">
        <div className="mb-3 px-3">
          <p
            className="text-[10px] font-semibold uppercase tracking-[0.3em]"
            style={{ color: 'var(--text-tertiary)' }}
          >
            Workspace
          </p>
        </div>

        <ul className="space-y-2 pb-2">
          {menuItems.map((item) => {
            const isActive = currentPage === item.id;
            const Icon = item.icon;
            const label = t(`sidebar.${item.id}`, { defaultValue: item.label });

            return (
              <li key={item.id}>
                <button
                  onClick={() => onNavigate(item.id)}
                  className="group relative flex w-full items-center gap-3 overflow-hidden rounded-2xl py-3 pr-4 pl-6 text-left transition-all duration-200 card-hover"
                  aria-current={isActive ? 'page' : undefined}
                  style={{
                    color: isActive ? 'var(--text-primary)' : 'var(--text-secondary)',
                    background: isActive
                      ? 'linear-gradient(135deg, rgba(255, 248, 240, 0.84), rgba(232, 223, 210, 0.92))'
                      : 'transparent',
                    border: `1px solid ${isActive ? 'rgba(201, 92, 52, 0.18)' : 'transparent'}`,
                  }}
                  onMouseEnter={(e) => {
                    if (!isActive) {
                      e.currentTarget.style.background = 'color-mix(in srgb, var(--bg-card-hover) 82%, transparent)';
                      e.currentTarget.style.borderColor = 'var(--border-secondary)';
                      e.currentTarget.style.color = 'var(--text-primary)';
                    }
                  }}
                  onMouseLeave={(e) => {
                    if (!isActive) {
                      e.currentTarget.style.background = 'transparent';
                      e.currentTarget.style.borderColor = 'transparent';
                      e.currentTarget.style.color = 'var(--text-secondary)';
                    }
                  }}
                >
                  {isActive && (
                    <motion.div
                      layoutId="activeIndicator"
                      className="absolute left-2 top-2 bottom-2 w-1 rounded-full bg-claw-500"
                      transition={{ type: 'spring', stiffness: 320, damping: 30 }}
                    />
                  )}

                  <div
                    className={clsx(
                      'flex h-10 w-10 shrink-0 items-center justify-center rounded-2xl border transition-colors duration-200',
                      isActive ? 'text-claw-600' : 'text-content-secondary group-hover:text-claw-500'
                    )}
                    style={{
                      backgroundColor: isActive ? 'rgba(201, 92, 52, 0.12)' : 'color-mix(in srgb, var(--bg-elevated) 72%, transparent)',
                      borderColor: isActive ? 'rgba(201, 92, 52, 0.18)' : 'var(--border-secondary)',
                    }}
                  >
                    <Icon size={18} />
                  </div>

                  <div className="min-w-0 flex-1">
                    <span className="block truncate text-sm font-medium">{label}</span>
                  </div>
                </button>
              </li>
            );
          })}
        </ul>
      </nav>

      <div
        className="relative shrink-0 p-4"
        style={{ borderTop: '1px solid var(--border-secondary)' }}
      >
        <div
          className="rounded-[24px] border p-4"
          style={{
            background: 'linear-gradient(180deg, color-mix(in srgb, var(--bg-card) 92%, transparent), color-mix(in srgb, var(--bg-elevated) 86%, transparent))',
            borderColor: 'var(--border-primary)',
          }}
        >
          <div className="flex flex-col gap-3">
            <div className="flex items-center gap-2">
              <div className={clsx('status-dot', isRunning ? 'running' : 'stopped')} />
              <span className="text-xs font-medium" style={{ color: 'var(--text-primary)' }}>
                {isRunning
                  ? t('sidebar.serviceRunning', { defaultValue: 'Service running' })
                  : t('sidebar.serviceStopped', { defaultValue: 'Service stopped' })}
              </span>
            </div>
            <div className="flex flex-wrap items-center gap-2">
              <span
                className="inline-flex shrink-0 rounded-full px-2.5 py-1 text-[10px] font-semibold"
                style={{
                  color: isRunning ? '#4f7c32' : '#8f4438',
                  backgroundColor: isRunning ? 'rgba(115, 154, 82, 0.14)' : 'rgba(201, 92, 52, 0.12)',
                }}
              >
                {isRunning ? 'Online' : 'Offline'}
              </span>
              <p className="text-xs" style={{ color: 'var(--text-tertiary)' }}>
                {t('sidebar.port', {
                  port: serviceStatus?.port ?? 18789,
                  defaultValue: `Port: ${serviceStatus?.port ?? 18789}`,
                })}
              </p>
            </div>
          </div>
        </div>
      </div>
    </aside>
  );
}
